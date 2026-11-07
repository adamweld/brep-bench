//! A block, the box (−6, −6, −8)–(36, 26, 0), with a groove cut into its top by a leaning bull-nose
//! mill carried by a frame riding the cubic Bézier curve (0, 0, 0), (10, 0, 0), (20, 20, 0),
//! (30, 20, 0): its origin the curve's point, its x the curve's tangent, its z up, the world's own
//! frame at the start. The mill, drawn in that frame, stands about the axis through c = (0, 0, −1)
//! along a = (sin 30°, 0, cos 30°), from c to c + 3a: radius 2, its end corner rounded to 0.5.
//!
//! The motion turns the mill as the curve turns: no slide, turn or screw, nor a path of them, so a
//! motion the kernel reads by evaluating it (`Trajectory::General`, its K2 road), the frame read
//! off the kernel's own B-spline.
use crate::sketch::block;
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::{tolerance, Frame, V};
use cad_kernel::nurbs::BSpline;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Motion, Path, Stretch, Trajectory};
use cad_kernel::util::interval::Interval;
use cad_kernel::util::space::{add, cross, dot, norm, scale, sub};
use cad_kernel::KernelError;
use std::f64::consts::{FRAC_PI_2, TAU};
use std::sync::Arc;

const STOCK_LO: V = [-6., -6., -8.];
const STOCK_HI: V = [36., 26., 0.];
const CONTROL: [V; 4] = [[0., 0., 0.], [10., 0., 0.], [20., 20., 0.], [30., 20., 0.]];
/// The middle of the mill's flat end, and how far its axis leans from upright toward +x, in
/// degrees.
const END: V = [0., 0., -1.];
const LEAN: f64 = 30.;
const RADIUS: f64 = 2.;
const CORNER: f64 = 0.5;
const LENGTH: f64 = 3.;
/// How closely the mill is meshed for the sweep's membership reads: a two-hundred-and-fiftieth of
/// its corner's radius.
const BAR: f64 = 0.002;

/// The bull-nose mill about the axis through `c` along the unit `a`, `e` a unit direction square
/// to it: its meridian in the half-plane from the axis toward `e` — out along its flat end, round
/// the quarter circle of its corner, up its side, back across its top and down its axis — turned
/// once about the axis.
fn mill(c: V, a: V, e: V) -> Result<Brep, String> {
    let at = |r: f64, h: f64| add(add(c, scale(e, r)), scale(a, h));
    let corner = Frame::new(at(RADIUS - CORNER, CORNER), cross(e, a), e);
    let edges = vec![
        ProfileEdge::Line { a: at(0., 0.), b: at(RADIUS - CORNER, 0.) },
        ProfileEdge::Arc { frame: corner, r: CORNER, span: Some([-FRAC_PI_2, 0.]) },
        ProfileEdge::Line { a: at(RADIUS, CORNER), b: at(RADIUS, LENGTH) },
        ProfileEdge::Line { a: at(RADIUS, LENGTH), b: at(0., LENGTH) },
        ProfileEdge::Line { a: at(0., LENGTH), b: at(0., 0.) },
    ];
    let meridian = Profile { origin: c, normal: cross(a, e), loops: vec![edges],
                             names: Vec::new() };
    revolve(&meridian, c, a, TAU)
}

/// The frame riding the curve: at `t` its origin the curve's point, its x the unit tangent, its z
/// up and its y = z × x, and their rates — the tangent's the curve's acceleration square to it
/// over its speed.
#[derive(Debug)]
struct Riding(BSpline);

impl Path for Riding {
    fn at(&self, t: f64) -> Result<Motion, KernelError> {
        let (point, rate, acceleration) = self.0.d2(t);
        let speed = norm(rate);
        if !(speed > 0.) { return Err(format!("the curve stands still at {t}").into()) }
        let x = scale(rate, 1. / speed);
        let dx = scale(sub(acceleration, scale(x, dot(x, acceleration))), 1. / speed);
        let up = [0., 0., 1.];
        let (y, dy) = (cross(up, x), cross(up, dx));
        // the turn's columns are the frame's axes
        let r = [[x[0], y[0], up[0]], [x[1], y[1], up[1]], [x[2], y[2], up[2]]];
        let dr = [[dx[0], dy[0], 0.], [dx[1], dy[1], 0.], [dx[2], dy[2], 0.]];
        Ok(Motion::of_parts(r, dr, point, rate))
    }

    /// What the motion does over any stretch, enclosed, as the kernel's membership reads its speed
    /// bound: the frame's origin within the poles' box (a Bézier curve lies in its control
    /// polygon's hull); its turn's entries within ±1 but the upright's; its speed within the
    /// longest of the hodograph's poles; its spin, the tangent's turning rate `|B′ × B″| / |B′|²`,
    /// within the longest of the second hodograph's poles over the least the rate reaches along
    /// the hodograph poles' mean direction (each rate a weighted mean of them).
    fn stretch(&self, _: f64, _: f64) -> Option<Stretch> {
        let h = hodograph();
        let along = h.iter().fold([0.; 3], |s, &p| add(s, p));
        let least = h.iter().map(|&p| dot(p, along) / norm(along)).fold(f64::INFINITY, f64::min);
        if !(least > 0.) { return None }
        let bend = (0..2).map(|i| norm(scale(sub(h[i + 1], h[i]), 2.))).fold(0., f64::max);
        let speed = h.iter().map(|&p| norm(p)).fold(0., f64::max);
        let (unit, zero, one) = (Interval::new(-1., 1.).ok()?, Interval::point(0.).ok()?,
                                 Interval::point(1.).ok()?);
        let turn = [[unit, unit, zero], [unit, unit, zero], [zero, zero, one]];
        Some(Stretch { point: hull(&CONTROL)?, turn, speed, spin: bend / least, origin: [0.; 3] })
    }
}

/// The box holding `points`, as intervals.
fn hull(points: &[V]) -> Option<[Interval; 3]> {
    let side = |k: usize| {
        let (lo, hi) = points.iter().fold((f64::INFINITY, f64::NEG_INFINITY),
                                          |(lo, hi), p| (lo.min(p[k]), hi.max(p[k])));
        Interval::new(lo, hi).ok()
    };
    Some([side(0)?, side(1)?, side(2)?])
}

/// The Bézier curve's hodograph's poles, `3 (P[i+1] − P[i])`: its rate is a weighted mean of them.
fn hodograph() -> [V; 3] {
    std::array::from_fn(|i| scale(sub(CONTROL[i + 1], CONTROL[i]), 3.))
}

pub fn build() -> Result<Brep, String> {
    let stock = block(STOCK_LO, STOCK_HI)?;
    let (s, c) = LEAN.to_radians().sin_cos();
    let tool = mill(END, [s, 0., c], [c, 0., -s])?;
    // the curve as cad-kernel holds a Bézier segment: a cubic B-spline on clamped knots
    let knots = vec![0., 0., 0., 0., 1., 1., 1., 1.];
    let curve = BSpline::new(3, knots, CONTROL.to_vec())?;
    let path = Trajectory::General(Arc::new(Riding(curve)));
    // the kernel's closed swept solid cut from the block on one arrangement: the sheet each
    // instant's contact sweeps, fitted, and the mill where it starts and where it stops
    let size = stock.size().max(tool.size());
    cut_from(&stock, &tool, &path, [0., 1.], BAR, tolerance(size))
        .map_err(|why| format!("the mill's sweep not built: {why:?}"))
}

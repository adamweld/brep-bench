//! A block, the box (−8, −8, −8)–(8, 8, 0), with a hole bored into its top by a bull-nose mill slid
//! without turning straight down by 8, stopping wholly inside. The mill stands about the axis
//! through c = (0, 0, 4) along a = (sin 30°, 0, cos 30°), from c to c + 3a: radius 2, its end
//! corner rounded to 0.5 (its flat end the disc of radius 1.5 about c).
//!
//! Refused by the kernel: the closed sweep's arrangement fails to meet one of the mill's circles
//! with the sheet its round's contact sweeps (`Unbuilt::Failed`: where a circle meets an extrusion
//! was not proved, the search reading more boxes than any may).
use crate::sketch::block;
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::{tolerance, Frame, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{add, cross, scale};
use std::f64::consts::{FRAC_PI_2, TAU};

const STOCK_LO: V = [-8., -8., -8.];
const STOCK_HI: V = [8., 8., 0.];
/// The middle of the mill's flat end, and how far its axis leans from upright toward +x, in
/// degrees.
const END: V = [0., 0., 4.];
const LEAN: f64 = 30.;
const RADIUS: f64 = 2.;
const CORNER: f64 = 0.5;
const LENGTH: f64 = 3.;
const PUSH: f64 = 8.;
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

pub fn build() -> Result<Brep, String> {
    let stock = block(STOCK_LO, STOCK_HI)?;
    let (s, c) = LEAN.to_radians().sin_cos();
    let tool = mill(END, [s, 0., c], [c, 0., -s])?;
    // the slide straight down: a screw that does not turn, advancing its whole length over a roll
    // of τ
    let down = [0., 0., -1.];
    let path = Trajectory::Twist(Screw { origin: END, axis: down, ratio: 0., advance: PUSH });
    // the kernel's closed swept solid cut from the block on one arrangement: the round, a band of a
    // torus's tube leaning to the slide between smooth seams, at both end poses and its contact's
    // arc swept into the sheet over that arc of its contour; the side's rulings into their tangent
    // planes; the flat end's rim into an oblique cylinder
    let size = stock.size().max(tool.size());
    cut_from(&stock, &tool, &path, [0., TAU], BAR, tolerance(size))
        .map_err(|why| format!("the mill's sweep not built: {why:?}"))
}

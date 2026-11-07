//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) whose floor is cut
//! by the flat end of a leaning bar turned about z, right-handed, from −140° to +140°. The end is
//! the rectangle {0 ≤ a cos 5° + b sin 5° ≤ 3, |b cos 5° − a sin 5°| ≤ 2} at
//! (10, 0, 0) + a u + b v, u = (cos 20°, 0, −sin 20°), v = (0, 1, 0); the bar is that rectangle
//! carried 3 along n = (sin 20°, 0, cos 20°).
use crate::sketch::polygon;
use cad_kernel::build::{prism, revolve};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{add, scale};
use std::f64::consts::FRAC_PI_3;

const UP: V = [0., 0., 1.];
/// Where the end's plane is drawn from: (a, b) = (0, 0).
const ORIGIN: V = [10., 0., 0.];
/// How far the bar leans out from upright, and how far its end is turned in its plane, in degrees.
const LEAN: f64 = 20.;
const TURNED: f64 = 5.;
/// The end: 3 across from its inner side, 2 either side of its middle line; the bar 3 long.
const WIDTH: f64 = 3.;
const HALF: f64 = 2.;
const LENGTH: f64 = 3.;
/// How far the turn carries the bar each way from where it is drawn, in degrees.
const TURN: f64 = 140.;

/// The ring sector: its section in the half-plane at 0° turned 60° about z.
fn sector() -> Result<Brep, String> {
    let section = polygon(&[[8., 0., -1.], [12., 0., -1.], [12., 0., 1.], [8., 0., 1.]]);
    revolve(&section, [0.; 3], UP, FRAC_PI_3)
}

pub fn build() -> Result<Brep, String> {
    let blank = sector()?;
    let (s, c) = LEAN.to_radians().sin_cos();
    let (u, v) = ([c, 0., -s], [0., 1., 0.]);
    // a corner of the end by its own coordinates, p across it from its inner side and q along it,
    // turned 5° in (a, b)
    let (s5, c5) = TURNED.to_radians().sin_cos();
    let corner = |p: f64, q: f64| {
        let (a, b) = (p * c5 - q * s5, p * s5 + q * c5);
        add(add(ORIGIN, scale(u, a)), scale(v, b))
    };
    // walked counter-clockwise in (a, b), so its normal is u × v = n, and swept 3 along it
    let end = polygon(&[corner(0., -HALF), corner(WIDTH, -HALF), corner(WIDTH, HALF),
                        corner(0., HALF)]);
    let bar = prism(&end, 0., LENGTH)?;
    // the turn about z: a screw that does not advance, a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: 0. });
    // the kernel's closed swept solid cut from the sector on one arrangement: the end's contact
    // line turned into a cone, the inner side's into a hyperboloid, every edge turned (a bar's
    // flat faces mesh exactly, at any bar)
    let size = blank.size().max(bar.size());
    let turn = TURN.to_radians();
    cut_from(&blank, &bar, &path, [-turn, turn], size, tolerance(size))
        .map_err(|why| format!("the bar's sweep not built: {why:?}"))
}

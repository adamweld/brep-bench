//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) cut by a bar turned
//! about z, right-handed, from −140° to +140°: the box whose inner side is the plane through
//! (10, 0, 0) square to e = (cos 20°, 0, −sin 20°), facing the axis squarely, 3 deep along e, 4
//! wide across y and 6 long along (sin 20°, 0, cos 20°) about (10, 0, 0).
use crate::sketch::polygon;
use cad_kernel::build::{prism, revolve};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{add, scale};
use std::f64::consts::FRAC_PI_3;

const UP: V = [0., 0., 1.];
/// The point of the bar's inner side on the plane y = 0 through its middle.
const INNER: V = [10., 0., 0.];
/// How far the bar leans out from upright, in degrees.
const LEAN: f64 = 20.;
const DEPTH: f64 = 3.;
const HALF_WIDTH: f64 = 2.;
const HALF_LENGTH: f64 = 3.;
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
    let along = [s, 0., c];
    // the inner side, a rectangle walked counter-clockwise in (y, along), so that its normal is
    // y × along = e, and swept 3 along e
    let at = |y: f64, l: f64| add(add(INNER, [0., y, 0.]), scale(along, l));
    let side = polygon(&[at(-HALF_WIDTH, -HALF_LENGTH), at(HALF_WIDTH, -HALF_LENGTH),
                         at(HALF_WIDTH, HALF_LENGTH), at(-HALF_WIDTH, HALF_LENGTH)]);
    let bar = prism(&side, 0., DEPTH)?;
    // the turn about z: a screw that does not advance, a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: 0. });
    // the kernel's closed swept solid cut from the sector on one arrangement: the inner side's
    // contact line, its line in the plane through the axis, turned into a cone, and every edge
    // turned (a box's flat faces mesh exactly, at any bar)
    let size = blank.size().max(bar.size());
    let turn = TURN.to_radians();
    cut_from(&blank, &bar, &path, [-turn, turn], size, tolerance(size))
        .map_err(|why| format!("the bar's sweep not built: {why:?}"))
}

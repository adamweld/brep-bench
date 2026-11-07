//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) notched by a drum,
//! the cylinder of radius 1 about the vertical line through (10, 0) from z = −0.5 to 0.5, turned
//! rigidly about z, right-handed, from −140° to +30°: it starts clear and stops halfway across.
use crate::sketch::polygon;
use cad_kernel::build::{disc, prism, revolve};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::FRAC_PI_3;

const UP: V = [0., 0., 1.];
const DRUM_FOOT: V = [10., 0., -0.5];
const DRUM_RADIUS: f64 = 1.;
const DRUM_HEIGHT: f64 = 1.;
/// The turn's start and stop, in degrees.
const FROM: f64 = -140.;
const TO: f64 = 30.;
/// How closely the drum is meshed for the sweep's membership reads: a five-hundredth of its radius.
const BAR: f64 = 0.002;

/// The ring sector: its section in the half-plane at 0° turned 60° about z.
fn sector() -> Result<Brep, String> {
    let section = polygon(&[[8., 0., -1.], [12., 0., -1.], [12., 0., 1.], [8., 0., 1.]]);
    revolve(&section, [0.; 3], UP, FRAC_PI_3)
}

pub fn build() -> Result<Brep, String> {
    let block = sector()?;
    let drum = prism(&disc(DRUM_FOOT, UP, DRUM_RADIUS), 0., DRUM_HEIGHT)?;
    // the turn about z: a screw that does not advance, a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: 0. });
    // the kernel's closed swept solid cut from the sector on one arrangement: a drum about an axis
    // along the turn's, its side's generators in the plane through both axes turned into
    // cylinders about z and its flat ends kept in their planes
    let size = block.size().max(drum.size());
    cut_from(&block, &drum, &path, [FROM.to_radians(), TO.to_radians()], BAR, tolerance(size))
        .map_err(|why| format!("the drum's sweep not built: {why:?}"))
}

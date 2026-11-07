//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) with a helical
//! groove milled through it by a drum, the cylinder of radius 1 about the vertical line through
//! (10, 0) from z = −0.5 to 0.5, screwed about z, right-handed and rising 0.5 a turn, from a turn
//! of −140° to +140°.
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
/// How far the screw rises a whole turn.
const RISE: f64 = 0.5;
/// How far the screw turns each way from where the drum is drawn, in degrees.
const TURN: f64 = 140.;
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
    // the screw about z: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: RISE });
    // the kernel's closed swept solid cut from the sector on one arrangement: the drum's side
    // touches along its generators in the plane through both axes, screwed into coaxial cylinders,
    // and its rims' discs screw into helical ramps
    let size = block.size().max(drum.size());
    let turn = TURN.to_radians();
    cut_from(&block, &drum, &path, [-turn, turn], BAR, tolerance(size))
        .map_err(|why| format!("the drum's sweep not built: {why:?}"))
}

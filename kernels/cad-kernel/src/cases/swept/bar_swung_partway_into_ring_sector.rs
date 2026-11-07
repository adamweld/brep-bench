//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) notched by the box
//! 10 ≤ x ≤ 11, −0.5 ≤ y ≤ 0.5, −0.6 ≤ z ≤ 0.6 turned rigidly about z, right-handed, from −140° to
//! +30°: it starts clear and stops halfway across.
use crate::sketch::{block, polygon};
use cad_kernel::build::revolve;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::FRAC_PI_3;

const UP: V = [0., 0., 1.];
const BAR_LO: V = [10., -0.5, -0.6];
const BAR_HI: V = [11., 0.5, 0.6];
/// The turn's start and stop, in degrees.
const FROM: f64 = -140.;
const TO: f64 = 30.;

/// The ring sector: its section in the half-plane at 0° turned 60° about z.
fn sector() -> Result<Brep, String> {
    let section = polygon(&[[8., 0., -1.], [12., 0., -1.], [12., 0., 1.], [8., 0., 1.]]);
    revolve(&section, [0.; 3], UP, FRAC_PI_3)
}

pub fn build() -> Result<Brep, String> {
    let blank = sector()?;
    let bar = block(BAR_LO, BAR_HI)?;
    // the turn about z: a screw that does not advance, a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: 0. });
    // the kernel's closed swept solid cut from the sector on one arrangement: every edge of the
    // box turned (a line along the axis into a cylinder, one square to it into a level plane) and
    // its faces at both end poses (a box's flat faces mesh exactly, at any bar)
    let size = blank.size().max(bar.size());
    cut_from(&blank, &bar, &path, [FROM.to_radians(), TO.to_radians()], size, tolerance(size))
        .map_err(|why| format!("the bar's sweep not built: {why:?}"))
}

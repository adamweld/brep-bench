//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) notched by a half
//! drum — the half y ≤ 0 of the solid cylinder of radius 1 about the vertical line through (10, 0)
//! from z = −0.5 to 0.5 — turned rigidly about z from −140° to +30°, flat face first.
use crate::sketch::polygon;
use cad_kernel::build::{disc, kept, prism, revolve};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const SECTOR: f64 = 60f64.to_radians();
const ROLL: [f64; 2] = [-140f64.to_radians(), 30f64.to_radians()];
/// How closely the half drum is meshed for the sweep's membership reads: a hundredth of its
/// radius.
const BAR: f64 = 0.01;

/// The ring sector about z: radii 8 to 12, heights −1 to 1, polar angles 0° to 60° — its section in
/// the half-plane y = 0, x > 0 turned right-handed about z.
fn sector() -> Result<Brep, String> {
    let section = polygon(&[[8., 0., -1.], [12., 0., -1.], [12., 0., 1.], [8., 0., 1.]]);
    revolve(&section, [0., 0., 0.], [0., 0., 1.], SECTOR)
}

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    // the drum, kept on the side y ≤ 0 of the plane through its axis
    let drum = prism(&disc([10., 0., -0.5], up, 1.), 0., 1.)?;
    let half = kept(&drum, [10., 0., 0.], [0., -1., 0.], 0.)?;
    // the turn about z: a screw that does not advance, turning a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0., 0., 0.], axis: up, ratio: 1., advance: 0. });
    // the kernel's swept solid cut from the sector on one arrangement: a cylinder about an axis
    // along the turn's, its flat faces planes
    let blank = sector()?;
    let size = blank.size().max(half.size());
    cut_from(&blank, &half, &path, ROLL, BAR, tolerance(size))
        .map_err(|why| format!("the half drum's sweep not built: {why:?}"))
}

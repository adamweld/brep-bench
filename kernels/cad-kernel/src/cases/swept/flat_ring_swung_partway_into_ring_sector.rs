//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) notched by a ring
//! lying flat, the torus about the vertical line through (10, 0) of radii 1 and 0.4 centred on
//! z = 0, turned rigidly about z from −140° to +30°: it starts clear of the block and stops halfway
//! across it.
use crate::sketch::polygon;
use cad_kernel::build::{revolve, torus};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const SECTOR: f64 = 60f64.to_radians();
const ROLL: [f64; 2] = [-140f64.to_radians(), 30f64.to_radians()];
/// How closely the ring is meshed for the sweep's membership reads: a hundredth of its tube's
/// radius.
const BAR: f64 = 0.004;

/// The ring sector about z: radii 8 to 12, heights −1 to 1, polar angles 0° to 60° — its section in
/// the half-plane y = 0, x > 0 turned right-handed about z.
fn sector() -> Result<Brep, String> {
    let section = polygon(&[[8., 0., -1.], [12., 0., -1.], [12., 0., 1.], [8., 0., 1.]]);
    revolve(&section, [0., 0., 0.], [0., 0., 1.], SECTOR)
}

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let ring = torus([10., 0., 0.], up, 1., 0.4)?;
    // the turn about z: a screw that does not advance, turning a radian a radian of the roll, the
    // ring drawn where the roll is nothing
    let path = Trajectory::Twist(Screw { origin: [0., 0., 0.], axis: up, ratio: 1., advance: 0. });
    // the kernel's swept solid cut from the sector on one arrangement: a torus about an axis along
    // the turn's
    let blank = sector()?;
    let size = blank.size().max(ring.size());
    cut_from(&blank, &ring, &path, ROLL, BAR, tolerance(size))
        .map_err(|why| format!("the ring's sweep not built: {why:?}"))
}

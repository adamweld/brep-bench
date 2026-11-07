//! The solid a ring fills slid without turning 15 along x, square to its own axis — farther than
//! its hole is wide: the torus about the z axis centred on the origin, of radii 6 and 2.
use cad_kernel::build::torus;
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const MAJOR: f64 = 6.;
const MINOR: f64 = 2.;
const SLIDE: f64 = 15.;
/// How closely the ring is meshed for the sweep's membership reads: a hundredth of its tube's
/// radius.
const BAR: f64 = 0.02;

pub fn build() -> Result<Brep, String> {
    let ring = torus([0., 0., 0.], [0., 0., 1.], MAJOR, MINOR)?;
    // the slide along x: a screw that does not turn, advancing its whole length over a roll of τ
    let path = Trajectory::Twist(Screw {
        origin: [0., 0., 0.], axis: [1., 0., 0.], ratio: 0., advance: SLIDE,
    });
    // the kernel's closed swept solid: a torus slid square to its axis, its top and bottom circles
    // kept in their planes, its meridians square to the slide swept into cylinders along it
    closed(&ring, &path, [0., TAU], BAR, tolerance(ring.size()))
        .map_err(|why| format!("the ring's sweep not built: {why:?}"))
}

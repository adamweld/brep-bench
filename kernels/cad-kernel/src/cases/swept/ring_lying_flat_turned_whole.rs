//! The solid a ring fills turned a whole turn about the z axis: the torus about the vertical line
//! through (8, 0), of radii 3 and 1, centred on z = 0.
use cad_kernel::build::torus;
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

/// How closely the ring is meshed for the sweep's membership reads: a hundredth of its tube's
/// radius.
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let ring = torus([8., 0., 0.], up, 3., 1.)?;
    // the turn about z: a screw that does not advance, a whole turn over a roll of τ
    let path = Trajectory::Twist(Screw { origin: [0., 0., 0.], axis: up, ratio: 1., advance: 0. });
    // the kernel's closed swept solid: a torus about an axis along the turn's, turned whole
    closed(&ring, &path, [0., TAU], BAR, tolerance(ring.size()))
        .map_err(|why| format!("the ring's sweep not built: {why:?}"))
}

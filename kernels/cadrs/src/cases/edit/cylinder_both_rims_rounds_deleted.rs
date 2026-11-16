//! The cylinder of radius 6 on the z axis from z = 0 to z = 10, both its rims rounded to radius 2;
//! then both rounds, quarter tori, deleted together, the ends and the side run on to meet in the
//! rims again: the cylinder.
//!
//! The cylinder is a circle extruded and both rims filleted, radius 2; then Delete face takes the
//! two rounds.
use crate::bench::{cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let c = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 6.0, 10.0)?;
    let top = k.edge_at(c, p3(0.0, -6.0, 10.0))?;
    let bottom = k.edge_at(c, p3(0.0, -6.0, 0.0))?;
    let rounded = fillet(k, c, &[top, bottom], 2.0)?;
    // each round at 45° round its tube, whose center line is the circle of radius 4 at z = 8 or 2
    let s = 2f64.sqrt();
    let rounds = k.faces_at(rounded, &[p3(0.0, -(4.0 + s), 8.0 + s), p3(0.0, -(4.0 + s), 2.0 - s)])?;
    deleted(k, rounded, &rounds)
}

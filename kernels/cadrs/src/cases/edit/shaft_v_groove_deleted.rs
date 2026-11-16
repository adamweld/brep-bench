//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a V-groove turned
//! into it (everything farther from the axis than 3 + |z − 10| cut away: two cones meeting at radius
//! 3 at z = 10), both cones deleted together: the shaft's cylinder below and above the groove is one
//! surface, which runs on across it.
//!
//! The body is made and the groove cut away by booleans; cadrs's Delete Face deletes the groove's
//! faces, picked by a point inside each.
use crate::bench::{cone, cut, cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let shaft = cylinder(k, p3(0.0, 0.0, 0.0), z, 5.0, 20.0)?;
    let below = cylinder(k, p3(0.0, 0.0, 7.0), z, 7.0, 3.0)?;
    let lower_cone = cone(k, p3(0.0, 0.0, 7.0), z, 6.0, 3.0, 3.0)?;
    let lower = cut(k, below, lower_cone)?;
    let above = cylinder(k, p3(0.0, 0.0, 10.0), z, 7.0, 3.0)?;
    let upper_cone = cone(k, p3(0.0, 0.0, 10.0), z, 3.0, 6.0, 3.0)?;
    let upper = cut(k, above, upper_cone)?;
    let half = cut(k, shaft, lower)?;
    let grooved = cut(k, half, upper)?;
    let groove = k.faces_at(grooved, &[p3(0.0, -4.0, 9.0), p3(0.0, -4.0, 11.0)])?;
    deleted(k, grooved, &groove)
}

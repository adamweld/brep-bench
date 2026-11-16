//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a round-bottomed
//! groove turned into it (the torus whose tube, radius 1.5, runs round the circle of radius 5 at
//! z = 10, cut away), the groove's one face deleted: the shaft's cylinder below and above the groove
//! is one surface, which runs on across it.
//!
//! The body is made and the groove cut away by booleans; cadrs's Delete Face deletes the groove's
//! faces, picked by a point inside each.
use crate::bench::{cut, cylinder, p3, torus, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let shaft = cylinder(k, p3(0.0, 0.0, 0.0), z, 5.0, 20.0)?;
    let ring = torus(k, p3(0.0, 0.0, 10.0), z, 5.0, 1.5)?;
    let grooved = cut(k, shaft, ring)?;
    let groove = k.faces_at(grooved, &[p3(0.0, -3.5, 10.0)])?;
    deleted(k, grooved, &groove)
}

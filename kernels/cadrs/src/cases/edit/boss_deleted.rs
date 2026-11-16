//! The cylinder of radius 8 on the z axis from z = −4 to z = 0 with the cylinder of radius 3 from z
//! = 0 to z = 8 standing on it; then the boss's side and top deleted, the plate's top closing over
//! its foot: the plate.
//!
//! The two cylinders are united; then Delete face takes the boss's side and top.
use crate::bench::{cylinder, p3, union, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let plate = cylinder(k, p3(0.0, 0.0, -4.0), z, 8.0, 4.0)?;
    let boss = cylinder(k, p3(0.0, 0.0, 0.0), z, 3.0, 8.0)?;
    let solid = union(k, plate, boss)?;
    let faces = k.faces_at(solid, &[p3(0.0, -3.0, 4.0), p3(0.0, 0.0, 8.0)])?;
    deleted(k, solid, &faces)
}

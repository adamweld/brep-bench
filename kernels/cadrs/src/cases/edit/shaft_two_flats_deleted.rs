//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with two opposite flats
//! (everything with |x| > 4 cut away), both flats deleted together: the cylinder closes over them.
//!
//! The body is made and cut by booleans; cadrs's Delete Face deletes the faces, picked by a point
//! inside each.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let shaft = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let right = block(k, p3(4.0, -6.0, -1.0), p3(6.0, 6.0, 21.0))?;
    let one = cut(k, shaft, right)?;
    let left = block(k, p3(-6.0, -6.0, -1.0), p3(-4.0, 6.0, 21.0))?;
    let body = cut(k, one, left)?;
    let faces = k.faces_at(body, &[p3(4.0, 0.0, 10.0), p3(-4.0, 0.0, 10.0)])?;
    deleted(k, body, &faces)
}

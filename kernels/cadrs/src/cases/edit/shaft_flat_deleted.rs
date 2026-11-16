//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a flat milled along
//! its whole length (everything with y < −4 cut away), the flat deleted: the shaft's cylinder, on
//! both its long sides, closes over it.
//!
//! The shaft is made and the box cut away; cadrs's Delete Face deletes the faces, picked by a point
//! inside each.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let shaft = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let tool = block(k, p3(-6.0, -6.0, -1.0), p3(6.0, -4.0, 21.0))?;
    let body = cut(k, shaft, tool)?;
    let faces = k.faces_at(body, &[p3(0.0, -4.0, 10.0)])?;
    deleted(k, body, &faces)
}

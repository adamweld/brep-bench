//! A cone frustum about the z axis, radius 6 at z = 0 to 3 at z = 20, with a flat milled along its
//! side (everything with y < −4 cut away), the flat deleted: the cone closes over it.
//!
//! The body is made and cut by booleans; cadrs's Delete Face deletes the faces, picked by a point
//! inside each.
use crate::bench::{block, cone, cut, p3, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let frustum = cone(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 6.0, 3.0, 20.0)?;
    let side = block(k, p3(-7.0, -7.0, -1.0), p3(7.0, -4.0, 21.0))?;
    let body = cut(k, frustum, side)?;
    let faces = k.faces_at(body, &[p3(0.0, -4.0, 5.0)])?;
    deleted(k, body, &faces)
}

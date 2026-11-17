//! A ball of radius 5 about the origin, its top cut flat at z = 3, the flat deleted: the ball closes
//! over it.
//!
//! The body is made and cut by booleans; cadrs's Delete Face deletes the faces, picked by a point
//! inside each.
use crate::bench::{ball, block, cut, p3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let top = block(k, p3(-6.0, -6.0, 3.0), p3(6.0, 6.0, 6.0))?;
    let body = cut(k, b, top)?;
    let faces = k.faces_at(body, &[p3(1.0, 0.0, 3.0)])?;
    deleted(k, body, &faces)
}

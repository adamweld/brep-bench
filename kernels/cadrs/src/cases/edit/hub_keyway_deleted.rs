//! A hub (the tube about the z axis from z = 0 to 20, outer radius 10, bore 4) with a keyway along
//! its bore (the box −1 ≤ x ≤ 1, −5 ≤ y ≤ 0 cut away), the keyway's floor and walls deleted: the
//! bore closes over it.
//!
//! The body is made and cut by booleans; cadrs's Delete Face deletes the faces, picked by a point
//! inside each.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let outer = cylinder(k, p3(0.0, 0.0, 0.0), z, 10.0, 20.0)?;
    let bore = cylinder(k, p3(0.0, 0.0, -1.0), z, 4.0, 22.0)?;
    let hub = cut(k, outer, bore)?;
    let key = block(k, p3(-1.0, -5.0, -1.0), p3(1.0, 0.0, 21.0))?;
    let body = cut(k, hub, key)?;
    let faces = k.faces_at(body, &[p3(0.0, -5.0, 10.0), p3(1.0, -4.5, 10.0), p3(-1.0, -4.5, 10.0)])?;
    deleted(k, body, &faces)
}

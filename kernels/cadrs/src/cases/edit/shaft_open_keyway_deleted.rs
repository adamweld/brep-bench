//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with an open keyway (the
//! box −1 ≤ x ≤ 1, y ≤ −3, z ≥ 8 cut away: 2 wide, from z = 8 out through the top end), its floor,
//! two walls and end deleted together: the shaft's cylinder and top close over it.
//!
//! The shaft is made and the box cut away; cadrs's Delete Face deletes the faces, picked by a point
//! inside each.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let shaft = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let tool = block(k, p3(-1.0, -6.0, 8.0), p3(1.0, -3.0, 21.0))?;
    let body = cut(k, shaft, tool)?;
    let faces = k.faces_at(body, &[p3(0.0, -3.0, 15.0), p3(1.0, -4.0, 15.0), p3(-1.0, -4.0, 15.0), p3(0.0, -4.0, 8.0)])?;
    deleted(k, body, &faces)
}

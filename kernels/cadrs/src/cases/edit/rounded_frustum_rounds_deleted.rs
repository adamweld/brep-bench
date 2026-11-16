//! A cone frustum's two rounded rims deleted: its ends and its cone meet again: the frustum from radius 6 at z = 0 to radius 3 at z = 6, both rims rounded to radius 3/2;
//! then delete both rounds together. cadrs's cone, both rims filleted; then Delete face.
use crate::bench::{cone, p3, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let frustum = cone(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 6.0, 3.0, 6.0)?;
    let rims = k.edges_at(frustum, &[p3(0.0, -3.0, 6.0), p3(0.0, -6.0, 0.0)])?;
    let rounded = fillet(k, frustum, &rims, 1.5)?;
    // each round deleted, halfway round its tube toward its old rim
    let rounds = k.faces_at(rounded, &[p3(0.0, -2.861545685053858, 5.77597621252806), p3(0.0, -4.848925229403217, 0.7114033318212998)])?;
    deleted(k, rounded, &rounds)
}

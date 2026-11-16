//! The frustum from radius 6 at z = 0 to radius 3 at z = 6, its top rim rounded to radius 1; then
//! the round deleted: the frustum. cadrs's cone primitive, its rim filleted; then Delete face.
use crate::bench::{cone, p3, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let c = cone(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 6.0, 3.0, 6.0)?;
    let rim = k.edge_at(c, p3(0.0, -3.0, 6.0))?;
    let rounded = fillet(k, c, &[rim], 1.0)?;
    let round = k.faces_at(rounded, &[p3(0.0, -2.9076971233692386, 5.850650808352039)])?;
    deleted(k, rounded, &round)
}

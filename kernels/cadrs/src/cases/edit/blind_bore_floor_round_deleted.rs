//! A block with a blind bore of radius 4 down to z = 4, the floor's edge rounded to radius 1; then
//! the round deleted. The bore a cylinder cut from the block, its floor edge filleted; then Delete
//! face.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(-10.0, -10.0, 0.0), p3(10.0, 10.0, 10.0))?;
    let bore = cylinder(k, p3(0.0, 0.0, 4.0), v3(0.0, 0.0, 1.0), 4.0, 7.0)?;
    let bored = cut(k, b, bore)?;
    let edge = k.edge_at(bored, p3(0.0, -4.0, 4.0))?;
    let rounded = fillet(k, bored, &[edge], 1.0)?;
    let round = k.faces_at(rounded, &[p3(0.0, -3.7071067811865475, 4.292893218813452)])?;
    deleted(k, rounded, &round)
}

//! A hemisphere with its rim rounded, shrunk past the round's radius: the half ball of radius 5 above z = 0, its rim rounded to radius 1, then
//! shrunk by 3/2. cadrs's ball in common with a block, filleted; then its
//! offset (shrunk).
use crate::bench::{ball, block, common, p3};
use crate::shared::fillet::fillet;
use crate::shared::offset::shrunk;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let above = block(k, p3(-6.0, -6.0, 0.0), p3(6.0, 6.0, 6.0))?;
    let half = common(k, b, above)?;
    let rim = k.edge_at(half, p3(0.0, -5.0, 0.0))?;
    let rounded = fillet(k, half, &[rim], 1.0)?;
    shrunk(k, rounded, 1.5)
}

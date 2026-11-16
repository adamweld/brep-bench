//! A hemisphere with its rim rounded, hollowed with walls thicker than the round: the half ball of radius 5 above z = 0, its rim rounded to radius 1, then
//! hollowed to walls 2 thick. cadrs's ball in common with a block, filleted; then its
//! shell (hollow).
use crate::bench::{ball, block, common, p3};
use crate::shared::fillet::fillet;
use crate::shared::shell::hollow;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let above = block(k, p3(-6.0, -6.0, 0.0), p3(6.0, 6.0, 6.0))?;
    let half = common(k, b, above)?;
    let rim = k.edge_at(half, p3(0.0, -5.0, 0.0))?;
    let rounded = fillet(k, half, &[rim], 1.0)?;
    hollow(k, rounded, 2.0)
}

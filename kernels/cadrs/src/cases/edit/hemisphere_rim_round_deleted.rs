//! The half of the ball of radius 5 above z = 0, its rim rounded to radius 1; then the round
//! deleted: the hemisphere. cadrs's ball less a block, the rim filleted; then Delete face.
use crate::bench::{ball, block, cut, p3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let below = block(k, p3(-6.0, -6.0, -6.0), p3(6.0, 6.0, 0.0))?;
    let half = cut(k, b, below)?;
    let rim = k.edge_at(half, p3(0.0, -5.0, 0.0))?;
    let rounded = fillet(k, half, &[rim], 1.0)?;
    let round = k.faces_at(rounded, &[p3(0.0, -4.620983133994804, 0.3363010339995828)])?;
    deleted(k, rounded, &round)
}

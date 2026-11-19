//! The chain of `boolean/chain-of-three-balls` (balls of radius 5, 4 and 3 about the origin,
//! (7, 0, 0) and (10, 4, 0), united one after another) with everything below the plane z = 0
//! through their centers cut away by a block.
use crate::bench::{block, cut, p3};
use crate::shared::boolean::chain_of_three_balls;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let chain = chain_of_three_balls(k)?;
    // everything below z = 0, with room to spare round the chain
    let below = block(k, p3(-20.0, -20.0, -20.0), p3(30.0, 30.0, 0.0))?;
    cut(k, chain, below)
}

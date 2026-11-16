//! A half ball shelled outward by 2, open at its dome. The upper half of the ball of radius 5 centred on the origin: the part with z ≥ 0, shelled outward to a wall
//! 2 thick, open at its dome: the slice of the ball between z = −2 and z = 0.
//!
//! The half ball is cadrs's sphere primitive less a block below z = 0; cadrs's Shell feature
//! (outward), the dome picked as the face to remove.
use crate::bench::{ball, block, cut, p3};
use crate::shared::shell::{shell_outward, Joins};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    // the ball's upper half: the ball less a block below z = 0
    let b = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let below = block(k, p3(-6.0, -6.0, -6.0), p3(6.0, 6.0, 0.0))?;
    let half = cut(k, b, below)?;
    shell_outward(k, half, &[p3(3.0, 0.0, 4.0)], 2.0, Joins::Sharp)
}

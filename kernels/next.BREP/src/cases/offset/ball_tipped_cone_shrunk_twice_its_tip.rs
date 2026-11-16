//! The cone on the z axis from its base of radius 6 at z = 0 to its apex at z = 12, the tip rounded
//! by a ball of radius 1 tangent to its side; a ball-tipped cone shrunk by twice its tip's radius.
//! The ball's face moved 2 inward is gone, and the moved side runs on to a sharp apex at z = 12 −
//! 2√5.
//!
//! The cone's frustum united with the ball, then the kernel's rounded `offset_solid`, inward.
use crate::solids::offset::{ball_tipped_cone, parallel};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Ok(parallel(&ball_tipped_cone()?, -2.0)?)
}

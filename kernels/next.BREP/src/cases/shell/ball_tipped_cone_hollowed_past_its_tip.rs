//! The cone on the z axis from its base of radius 6 at z = 0 to its apex at z = 12, the tip rounded
//! by a ball of radius 1 tangent to its side; a ball-tipped cone hollowed to a wall thicker than
//! its tip's radius. The ball's face moved 1.5 inward is gone, and the moved side runs on to a
//! sharp apex at z = 12 − 1.5√5.
//!
//! The cone's frustum united with the ball, then the kernel's shell with no opening
//! (`offset_shell_closed`).
use crate::solids::offset::ball_tipped_cone;
use crate::Refusal;
use brep_kernel::{offset_shell_closed, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    Ok(offset_shell_closed(&ball_tipped_cone()?, 1.5)?.solid)
}

//! The cone on the z axis from its base of radius 6 at z = 0 to its apex at z = 12, the tip rounded
//! by a ball of radius 1 tangent to its side; a ball-tipped cone shelled open at its base, the wall
//! thicker than its tip's radius. The ball's face moved 1.5 inward is gone, and the moved side runs
//! on to a sharp apex at z = 12 − 1.5√5.
//!
//! The cone's frustum united with the ball, then the kernel's shell (`offset_shell`), open at the
//! base.
use crate::solids::offset::{ball_tipped_cone, shell};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    // open at the base, picked by a point of it
    shell(&ball_tipped_cone()?, &[p(1.0, 0.0, 0.0)], 1.5)
}

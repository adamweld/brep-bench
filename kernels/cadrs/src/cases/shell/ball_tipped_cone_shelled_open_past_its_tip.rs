//! The cone on the z axis from its base of radius 6 at z = 0 to its apex at z = 12, the tip rounded
//! by a ball of radius 1 tangent to its side; a ball-tipped cone shelled open at its base, the wall
//! thicker than its tip's radius. The ball's face moved 1.5 inward is gone, and the moved side runs
//! on to a sharp apex at z = 12 − 1.5√5.
//!
//! A cone frustum and a ball united, then cadrs's Shell, open at the base.
use crate::bench::p3;
use crate::shared::offset::ball_tipped_cone;
use crate::shared::shell::{shell_inward, Joins};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let body = ball_tipped_cone(k)?;
    // open at the base, picked by a point of it
    shell_inward(k, body, &[p3(1.0, 0.0, 0.0)], 1.5, Joins::Rounded)
}

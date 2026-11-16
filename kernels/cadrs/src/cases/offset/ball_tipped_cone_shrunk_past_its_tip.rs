//! The cone on the z axis from its base of radius 6 at z = 0 to its apex at z = 12, the tip rounded
//! by a ball of radius 1 tangent to its side; a ball-tipped cone shrunk past its tip's radius. The
//! ball's face moved 1.5 inward is gone, and the moved side runs on to a sharp apex at z = 12 −
//! 1.5√5.
//!
//! A cone frustum and a ball united, then cadrs's offset of the whole solid, inward.
use crate::shared::offset::{ball_tipped_cone, shrunk};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let body = ball_tipped_cone(k)?;
    shrunk(k, body, 1.5)
}

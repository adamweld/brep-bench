//! The ball-tipped cone of `offset/ball-tipped-cone-shrunk-past-its-tip`, its tip a ball of radius
//! 1, shrunk by 2: every point at least 2 from its outside, the kernel's offset inward with the
//! moved faces that cross trimmed and merged.
use super::ball_tipped_cone_shrunk_past_its_tip::ball_tipped_cone;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&ball_tipped_cone()?, -2.0)
}

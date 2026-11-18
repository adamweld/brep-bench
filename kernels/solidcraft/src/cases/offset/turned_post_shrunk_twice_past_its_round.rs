//! The turned post of `offset/turned-post-shrunk-past-its-round`, its shoulder rounded to radius
//! 1, shrunk by 2: every point at least 2 from its outside, the kernel's offset inward with the
//! moved faces that cross trimmed and merged.
use super::turned_post_shrunk_past_its_round::turned_post;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&turned_post()?, -2.0)
}

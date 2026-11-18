//! The puck of `offset/rounded-puck-shrunk-by-its-rounds`, its rims rounded to radius 2, shrunk
//! by 3: every point at least 3 from its outside, the kernel's offset inward with the moved faces
//! that cross trimmed and merged.
use super::rounded_puck_shrunk_by_its_rounds::rounded_puck;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&rounded_puck()?, -3.0)
}

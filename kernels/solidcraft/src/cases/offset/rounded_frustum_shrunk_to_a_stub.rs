//! The frustum of `offset/rounded-frustum-shrunk-past-its-rounds`, both rims rounded to radius
//! 1.5, shrunk by 2.5: every point at least 2.5 from its outside, the kernel's offset inward with
//! the moved faces that cross trimmed and merged.
use super::rounded_frustum_shrunk_past_its_rounds::frustum_rounded;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&frustum_rounded(&[(6.0, 0.0), (3.0, 6.0)])?, -2.5)
}

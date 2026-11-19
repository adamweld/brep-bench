//! The cone frustum on the z axis from radius 6 at z = 0 to radius 3 at z = 6, its top rim rounded
//! to radius 1.5 and its bottom rim sharp, shrunk by 2: every point at least 2 from its outside,
//! the kernel's offset inward with the moved faces that cross trimmed and merged.
use super::rounded_frustum_shrunk_past_its_rounds::frustum_rounded;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&frustum_rounded(&[(3.0, 6.0)])?, -2.0)
}

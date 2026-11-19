//! The cone boss on a plate of `offset/filleted-cone-boss-grown-by-its-fillet`, its concave edge
//! filleted to radius 1, grown by 2: every point within 2 of it, the kernel's offset with its
//! convex edges rounded.
use super::filleted_cone_boss_grown_by_its_fillet::filleted_cone_boss;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&filleted_cone_boss(1.0)?, 2.0)
}

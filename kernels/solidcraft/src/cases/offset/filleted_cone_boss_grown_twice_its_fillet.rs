//! The cone boss on a plate of `offset/filleted-cone-boss-grown-by-its-fillet`, its concave edge
//! filleted to radius 1.5, grown by 3: every point within 3 of it, the kernel's offset with its
//! convex edges rounded.
use super::filleted_cone_boss_grown_by_its_fillet::filleted_cone_boss;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&filleted_cone_boss(1.5)?, 3.0)
}

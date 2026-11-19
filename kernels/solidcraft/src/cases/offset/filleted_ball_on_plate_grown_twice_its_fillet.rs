//! The ball on a plate of `offset/filleted-ball-on-plate-grown-past-its-fillet`, its concave edge
//! filleted to radius 1.5, grown by 3: every point within 3 of it, the kernel's offset with its
//! convex edges rounded.
use super::filleted_ball_on_plate_grown_past_its_fillet::filleted_ball_on_plate;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&filleted_ball_on_plate(1.5)?, 3.0)
}

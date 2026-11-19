//! Refused. Split by its great disc square to the helix, the ball's back half first reaches a
//! point, and its front half leaves one, only through the part of that disc moving forward, where
//! 1.5x + (3/2π)² ≥ 0 (a sphere turned about any line moves along itself), so the sweep is the ball
//! at its two ends with that part's screw sweep. But the helix turns tighter than the ball, and
//! that sweep runs through itself: the disc reaches 3.8 along the axis against a lead of 3, and
//! near the axis it passes the same points again within a fraction of a turn, so it is no solid
//! the kernel's helical sweep (`sweep_helix`) makes, and the solid's boundary is that sweep's
//! envelope trimmed where it meets itself. solidcraft-kernel has no sweep of a solid and no envelope, and that trimmed envelope
//! computed here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no sweep of a solid and no envelope: the helix turns tighter than \
         the ball, so the screw sweep of the ball's forward-moving great disc runs through itself, \
         and the solid's boundary is that sweep's envelope trimmed where it meets itself"
        .into())
}

//! Refused. The groove is what the spheroid, leaning at the helix angle, passes through as it is
//! screwed along the rod. Its wall is the envelope of the spheroid over the screw, touching it
//! along a curve that is none of its plane sections, so no section of the wheel screwed by the
//! kernel's helical sweep (`sweep_helix`) makes it. solidcraft-kernel has no sweep of a solid and
//! no envelope, and that curve computed here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no sweep of a solid and no envelope: `sweep_helix` screws a \
         planar profile, but the groove's wall is the envelope of the leaning spheroid over the \
         screw, touching it along no plane section of it"
        .into())
}

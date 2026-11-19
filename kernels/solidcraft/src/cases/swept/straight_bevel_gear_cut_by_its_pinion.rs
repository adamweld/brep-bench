//! Refused. Seen from the blank the tooth turns about the x axis and the z axis at once, and the
//! tooth space's flanks are the envelope of its planar flanks over that motion: a cone over a
//! spherical curve, no plane involute. solidcraft-kernel has no sweep of a solid and no envelope,
//! and its sweeps of a profile follow a line, a screw, a revolve or a path's own frame, none of
//! which is this motion; an envelope computed here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no sweep of a solid and no envelope: the tooth space's flanks \
         are the envelope of the pinion tooth's flanks under two turns at once, which no sweep \
         of a profile along a line, a screw, a revolve or a path makes"
        .into())
}

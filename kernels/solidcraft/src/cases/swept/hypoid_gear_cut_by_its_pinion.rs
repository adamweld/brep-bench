//! Refused. Seen from the blank the tooth turns about two skew axes at once, and the tooth space's
//! flanks are the envelope of its planar flanks over that motion. solidcraft-kernel has no sweep of
//! a solid and no envelope, and its sweeps of a profile follow a line, a screw, a revolve or a
//! path's own frame, none of which is this motion; an envelope computed here would be geometry made
//! outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or an envelope: the tooth space's flanks \
         are the envelope of the pinion tooth's flanks under turns about two skew axes at once, \
         which no sweep of a profile along a line, a screw, a revolve or a path makes"
        .into())
}

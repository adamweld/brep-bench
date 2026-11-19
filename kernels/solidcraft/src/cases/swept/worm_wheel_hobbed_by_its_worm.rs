//! Refused. Seen from the wheel the worm is a rack slid along its axis while the wheel turns, and
//! the wheel's flanks are the envelope of the thread's helicoid flanks over that motion, involutes
//! only in the mid-plane. solidcraft-kernel has no sweep of a solid and no envelope, and its sweeps
//! of a profile follow a line, a screw, a revolve or a path's own frame, none of which is this
//! motion; an envelope computed here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no sweep of a solid and no envelope: the wheel's flanks are the \
         envelope of the worm's thread as it slides along its axis while the wheel turns, which \
         no sweep of a profile along a line, a screw, a revolve or a path makes"
        .into())
}

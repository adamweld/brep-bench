//! Refused. The cube moves by two turns about skew axes at once, so no line of it stays fixed and
//! the faces it leads with change as it tumbles; its sweep's boundary is the envelope of its
//! places. solidcraft-kernel has no sweep of a solid and no envelope, and its sweeps of a profile
//! follow a line, a screw, a revolve or a path's own frame, none of which carries even one face of
//! the cube through this motion; an envelope computed here would be geometry made outside the
//! kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or an envelope: the tumbling cube's sweep \
         is bounded by the envelope of its places, and no sweep of a profile along a line, a \
         screw, a revolve or a path carries even a face of it through two turns about skew axes"
        .into())
}

//! Refused. The ring, its axis leaning 35° from upright, slides straight down: the hole's wall and
//! the core's are the torus's outer and inner outlines seen from above carried down, the curves
//! where its normal is level, which are none of its circles. solidcraft-kernel has no sweep of a
//! solid and no envelope, and its sweeps carry plane profiles only, so those outlines computed
//! here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or an envelope: the walls are the leaning torus's \
         outlines along the push, the curves where its normal is level, none of its circles"
        .into())
}

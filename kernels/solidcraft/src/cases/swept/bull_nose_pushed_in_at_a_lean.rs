//! Refused. The bull-nose mill, its axis leaning 30° from upright, slides straight down: the hole's
//! wall is the mill's outline seen from above carried down, and on the mill's rounded corner (a
//! quarter torus) that outline runs where the torus's normal is level, a curve that is none of
//! its circles. solidcraft-kernel has no sweep of a solid and no envelope, and its sweeps carry
//! plane profiles only, so that outline computed here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no sweep of a solid and no envelope: the hole's wall is the leaning \
         mill's outline along the push, which on its quarter-torus corner is the curve where the \
         torus's normal is level, none of its circles"
        .into())
}

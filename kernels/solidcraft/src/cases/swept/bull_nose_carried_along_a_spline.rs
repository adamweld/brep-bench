//! Refused. The bull-nose mill leans 30° from upright and is carried by a frame that turns with
//! the curve's tangent, so it turns relative to its own axis as it goes: no screw or turn about
//! its axis carries it, and the groove's walls are the envelope of the leaning mill's side and
//! round over the motion, touching it along a curve that moves on it from instant to instant.
//! solidcraft-kernel has no sweep of a solid and no envelope, and its sweeps of a profile carry a
//! plane section along the path, none of which is that envelope; computed here it would be
//! geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or an envelope: the leaning mill turns with the \
         curve, so the groove's walls are the envelope of its side and round over the motion, \
         which no plane section of it swept along the curve makes"
        .into())
}

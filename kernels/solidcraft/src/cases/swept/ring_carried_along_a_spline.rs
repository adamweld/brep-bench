//! Refused. The ring stands across the curve like a wheel, reaching 2 ahead of and behind the
//! frame's origin, and turns with the curve's tangent, so as the curve bends its rim and its inner
//! side come into contact: the groove's walls are the envelope of the torus over the motion, none
//! of its circles. solidcraft-kernel has no sweep of a solid and no envelope, and its sweeps of a
//! profile carry a plane section along the path, which is not that envelope; computed here it
//! would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or an envelope: the wheel-like ring turns with the \
         curve, so the groove's walls are the envelope of the torus over the motion, which no \
         plane section of it swept along the curve makes"
        .into())
}

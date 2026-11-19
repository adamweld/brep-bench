//! Refused. The key is carried by a frame that turns with the curve's tangent, and it reaches 1.5
//! ahead of and behind the frame's origin, so where the curve bends its corners dig past its
//! sides: the groove's walls are made by the paths of its corners and the envelopes of its faces
//! over the motion, changing from face to face and edge to edge. solidcraft-kernel has no sweep
//! of a solid and no envelope, and its sweeps of a profile carry a plane section along the path,
//! which makes none of those walls; computed here they would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no sweep of a solid and no envelope: the key turns with the curve and \
         its corners dig past its sides where it bends, so the groove's walls are its corners' \
         paths and its faces' envelopes, which no plane section swept along the curve makes"
        .into())
}

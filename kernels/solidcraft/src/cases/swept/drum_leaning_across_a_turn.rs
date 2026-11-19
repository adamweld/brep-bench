//! Refused. The drum's axis leans 25° in the plane y = 1.2, a line skew to the z axis it turns
//! about. Its side touches its sweep along two curves that are none of its lines or circles, and
//! its rims, circles leaning to the turn's axis, sweep rings whose section is no circle: the
//! channel's walls are the revolves of an envelope. solidcraft-kernel has no sweep of a solid and
//! no envelope, and that contact computed here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or an envelope: the drum, its axis skew to the \
         turn's, touches its sweep along curves that are none of its lines or circles, so the \
         channel is bounded by an envelope"
        .into())
}

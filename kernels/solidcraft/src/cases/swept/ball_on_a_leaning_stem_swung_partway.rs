//! Refused. The tool is a ball on a stem, a cylinder whose axis leans 20° across the turn: a line
//! skew to the z axis. Turned about the z axis, a cylinder about a skew line touches its sweep
//! along curves that are none of its lines or circles (where its normal is square to the turn's
//! velocity), so the notch's walls are the revolves of an envelope. solidcraft-kernel has no
//! sweep of a solid and no envelope, and that contact computed here would be geometry made
//! outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no sweep of a solid and no envelope: the stem, a cylinder about a \
         line skew to the turn's axis, touches its sweep along curves that are none of its lines \
         or circles, so what it cuts is bounded by an envelope"
        .into())
}

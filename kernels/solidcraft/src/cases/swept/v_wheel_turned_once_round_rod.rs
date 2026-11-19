//! Refused. A whole turn about the z axis sweeps the wheel's shadow on a half-plane through the
//! axis, revolved; but the wheel's axis leans 20° across the turn, a line skew to the z axis, so
//! its flanks touch along curves that are none of its circles and the shadow is bounded by their
//! envelope (and by the shadow of its sharp rim, no conic). solidcraft-kernel has no sweep of a
//! solid and no envelope, and that shadow computed here would be geometry made outside the
//! kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or an envelope: turned whole about the z axis, \
         the leaning wheel sweeps the revolve of its shadow through the axis, whose edge is an \
         envelope of its flanks' circles"
        .into())
}

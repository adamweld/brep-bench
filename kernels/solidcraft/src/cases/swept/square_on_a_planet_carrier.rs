//! Refused. The bar moves by two turns at once, about the z axis and about its own centre carried
//! round it, so no line of it stays fixed: its section sweeps a region bounded by the envelopes
//! of its sides and the trochoids its corners trace. solidcraft-kernel has no sweep of a solid,
//! no envelope and no trochoid, and those computed here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no sweep of a solid and no envelope: the bar turns about its own \
         centre as that is carried round the z axis, so its sweep is bounded by the envelopes of \
         its sides and the trochoids of its corners"
        .into())
}

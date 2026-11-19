//! Refused. The spring is itself a swept solid, a ball's sweep along a helix, and pushed aslant it
//! cuts from each level of the plate its shadow along the push: a region bounded by the
//! silhouettes of its coils, the curves where the tube's normal is square to the push, whose
//! loops overlap. solidcraft-kernel has no sweep of a solid and no envelope; the spring's sweep is
//! everything within 0.4 of the helix slid along the push, a ruled sheet grown by a ball, which
//! none of its sweeps of a profile makes, and those silhouettes computed here would be geometry
//! made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no sweep of a solid and no envelope: the hole is the spring's shadow \
         along the push, bounded by the silhouettes of its helical coils, which no sweep of a \
         profile makes"
        .into())
}

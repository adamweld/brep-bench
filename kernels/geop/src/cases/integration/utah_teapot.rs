//! Martin Newell's teapot as a solid: the union of four solids bounded by his 32 bicubic Bézier
//! patches, given by their control points, and flat faces closing them. Refused: geop has no
//! surface given by its control net. Its sketches take any NURBS curve, but its surfaces come only
//! from operations — extrude, revolve, sweep, a loft or network surface that interpolates curves,
//! a ruled or Coons boundary surface — and none of them makes a tensor-product patch from given
//! control points.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop has no surface given by its control net: a sketch takes any NURBS curve, but its surfaces come only from operations (extrude, revolve, sweep, a loft or network surface interpolating curves, a ruled or Coons boundary surface), none of which makes the teapot's bicubic Bézier patches from their control points".into())
}

//! The solid over the square from (0, 0) to (10, 10) under the bilinear Coons patch of four
//! quadratic Bézier arcs, with upright walls and a flat floor.
//!
//! Refused: springrep 1.0.1 has no fill of a loop of curves. Its free-form surfaces come from a
//! loft through planar sections (`loft::loft`, which skins the sections' outlines across them)
//! and a height field given by its coefficients (`primitive::slab`); nothing takes four boundary
//! curves and makes the surface they bound. A loft between the arcs on y = 0 and y = 10 is ruled
//! (or smooth) across them, and so misses the arcs on x = 0 and x = 10; writing the patch's net as
//! a slab's coefficients would be the surface built outside the kernel.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no fill of a loop of curves (no Coons patch): a loft skins planar sections \
         and a slab takes its surface's coefficients, so the patch of four boundary arcs cannot be \
         made from the arcs"
        .into())
}

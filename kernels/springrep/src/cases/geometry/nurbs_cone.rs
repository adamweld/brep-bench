//! A cone of radius 5 and height 10 whose side is one rational B-spline face, quadratic around and
//! linear up, collapsing to a pole at the apex, closed by a flat disk. springrep has the surface
//! (`NurbsSurface::new`), but no public way to make a face, let alone a solid, from a given
//! surface: its face and loop builders are crate-private, and only its own operations and STEP
//! import make NURBS faces. A loft from the circle to a point is not one of them (a loft's sections
//! are outlines), and its surfaces are its own, not this net.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no public way to make a face or a solid from a given NURBS surface (its \
         face builders are crate-private; only STEP import makes faces of given surfaces)"
        .into())
}

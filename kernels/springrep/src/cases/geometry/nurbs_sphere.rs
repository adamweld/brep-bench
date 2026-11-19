//! A ball of radius 5 bounded by one biquadratic rational B-spline face with a pole at each end.
//! springrep has the surface (`NurbsSurface::new` takes the control net, its weights and knots),
//! but no public way to make a face, let alone a solid, from a surface: its face and loop builders
//! are crate-private, and only its own operations (sweeps, lofts, slabs, affine maps) and STEP
//! import make NURBS faces. None of those makes this net: a slab is a height field, a loft skins
//! planar sections, and the affine image of a sphere is springrep's own rational form, not this
//! one.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no public way to make a face or a solid from a given NURBS surface (its \
         face builders are crate-private; only STEP import makes faces of given surfaces)"
        .into())
}

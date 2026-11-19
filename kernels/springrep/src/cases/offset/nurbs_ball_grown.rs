//! A ball of radius 5 bounded by one biquadratic rational B-spline face with a pole at each end,
//! grown by 2. springrep's offset would take the face (it fits a certified offset of a spline
//! face), but springrep has no public way to make the ball: a face, let alone a solid, from a given
//! NURBS surface (its face builders are crate-private, and only STEP import makes faces of given
//! surfaces). See `geometry/nurbs-sphere`.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no public way to make a face or a solid from a given NURBS surface (its \
         face builders are crate-private; only STEP import makes faces of given surfaces)"
        .into())
}

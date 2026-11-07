//! The elliptic cylinder of `geometry/elliptic-cylinder` (x²/25 + y²/9 ≤ 1 from z = 0 to 10)
//! shrunk by 1: every point of it at least 1 inside it.
//!
//! Refused, by the kernel: "face 0 (extrusion) is not swept along its normal: a extrusion has no
//! offset in closed form". cad-kernel has no ellipse in a profile, so the side is the rational
//! B-spline ellipse extruded, and `offset::shrunk` moves a face only by its surface's offset in
//! closed form (planes, cylinders, cones, spheres, tori); an ellipse's offset is no conic.
use crate::cases::geometry::elliptic_cylinder;
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    shrunk(&elliptic_cylinder::build()?, 1., Join::Round)
}

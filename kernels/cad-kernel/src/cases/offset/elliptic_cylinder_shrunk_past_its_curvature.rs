//! The elliptic cylinder of `geometry/elliptic-cylinder` (x²/25 + y²/9 ≤ 1 from z = 0 to 10)
//! shrunk by 2.5, past its least radius of curvature b²/a = 1.8: its side's offset swallowtails
//! near the ends of the long axis, and the solid of every point at least 2.5 inside keeps only its
//! part clear of them, meeting itself in a sharp upright edge at each end.
//!
//! Refused, by the kernel: "face 0 (extrusion) is not swept along its normal: a extrusion has no
//! offset in closed form". cad-kernel has no ellipse in a profile, so the side is the rational
//! B-spline ellipse extruded, and `offset::shrunk` moves a face only by its surface's offset in
//! closed form (planes, cylinders, cones, spheres, tori); an ellipse's offset is no conic.
use crate::cases::geometry::elliptic_cylinder;
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    shrunk(&elliptic_cylinder::build()?, 2.5, Join::Round)
}

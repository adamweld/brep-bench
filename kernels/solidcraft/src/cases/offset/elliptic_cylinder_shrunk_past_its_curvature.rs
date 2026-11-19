//! The elliptic cylinder of `geometry/elliptic-cylinder` (x²/25 + y²/9 ≤ 1 from z = 0 to z = 10)
//! shrunk by 2.5, more than the ellipse's least radius of curvature (1.8): the moved side folds
//! into swallowtails at the ends of the long axis, which the kernel's offset inward trims away
//! where the moved side crosses itself (its self-intersections merged).
use crate::shared::geometry::elliptic_cylinder;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&elliptic_cylinder(5.0, 3.0, 0.0, 10.0)?, -2.5)
}

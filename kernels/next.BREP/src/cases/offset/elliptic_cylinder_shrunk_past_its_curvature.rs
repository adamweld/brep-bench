//! The elliptic cylinder of `geometry/elliptic-cylinder`, x²/25 + y²/9 ≤ 1 from z = 0 to z = 10,
//! shrunk by 2.5 to its parallel body by the kernel's offset: past the ellipse's tightest
//! curvature, so the moved side's swallowtails must be trimmed where it crosses itself.
use crate::cases::geometry::r#elliptic_cylinder;
use crate::solids::offset::parallel;
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Ok(parallel(&elliptic_cylinder::build()?, -2.5)?)
}

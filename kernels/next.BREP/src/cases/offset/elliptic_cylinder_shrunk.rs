//! The elliptic cylinder of `geometry/elliptic-cylinder`, x²/25 + y²/9 ≤ 1 from z = 0 to z = 10,
//! shrunk by 1 to its parallel body by the kernel's offset: its side moved in along its normals,
//! its ends to z = 1 and z = 9, its edges sharp.
use crate::cases::geometry::r#elliptic_cylinder;
use crate::solids::offset::parallel;
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Ok(parallel(&elliptic_cylinder::build()?, -1.0)?)
}

//! The ball of radius 5 about the origin bounded by one NURBS face, the standard biquadratic
//! sphere of `geometry/nurbs-sphere`, grown by 2 by the kernel's offset of that face along its
//! normal, through its poles.
use crate::cases::geometry::r#nurbs_sphere;
use crate::solids::offset::parallel;
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Ok(parallel(&nurbs_sphere::build()?, 2.0)?)
}

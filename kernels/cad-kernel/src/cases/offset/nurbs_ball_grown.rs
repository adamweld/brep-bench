//! The ball of radius 5 about the origin bounded by the one NURBS face of `geometry/nurbs-sphere`
//! (a 9 × 5 rational net, its first and last rows each collapsed to a pole), grown by 2: its face
//! offset out along its normal through the poles, the ball of radius 7.
//!
//! Refused, by the kernel: "`east` (bspline) is not swept along its normal: a bspline has no
//! offset in closed form". `offset::grown` moves a face only by its surface's offset in closed
//! form, and does not read the rational net as the sphere it is.
use crate::cases::geometry::nurbs_sphere;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    grown(&nurbs_sphere::build()?, 2., Join::Round)
}

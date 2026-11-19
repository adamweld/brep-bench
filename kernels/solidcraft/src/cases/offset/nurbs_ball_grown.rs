//! The ball of radius 5 of `geometry/nurbs-sphere`, its whole surface one NURBS face, grown by 2:
//! its one face moved 2 out along its normal, through the poles, onto the surface's offset. The
//! ball has no edge to round. ops's offset of a body's faces, `offset_sheet` (the kernel's offset
//! with the convex edges extended), fails on it with the kernel's bare status 1001, so this is the
//! kernel's offset with them rounded.
use crate::cases::geometry::nurbs_sphere;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&nurbs_sphere::build()?, 2.0)
}

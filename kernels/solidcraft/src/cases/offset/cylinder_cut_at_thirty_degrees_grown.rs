//! A cylinder of radius 3 about the z axis from z = −10 up to the plane z = x·tan 30°, grown by 1:
//! every point within 1 of it, the kernel's offset with its convex edges rounded. The cylinder is
//! that of `offset/slant-cut-cylinder-grown`, cut at 30° instead of 45°.
use super::slant_cut_cylinder_grown::slant_cut_cylinder;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&slant_cut_cylinder(30.0)?, 1.0)
}

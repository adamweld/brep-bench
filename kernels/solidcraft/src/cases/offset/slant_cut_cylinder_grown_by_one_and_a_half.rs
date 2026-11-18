//! The cylinder of `offset/slant-cut-cylinder-grown`, radius 3 about the z axis from z = −10 up to
//! the plane z = x·tan 45°, grown by 1.5: every point within 1.5 of it, the kernel's offset
//! with its convex edges rounded.
use super::slant_cut_cylinder_grown::slant_cut_cylinder;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&slant_cut_cylinder(45.0)?, 1.5)
}

//! A rod threaded by a bar whose section is a closed periodic cubic B-spline, carried along a
//! screw.
//!
//! Refused: the kernel's sweep of a solid takes a solid of revolution along a screw, and its prism
//! outlines are lines and circular arcs only; the spline bar is neither, and its contact under the
//! screw is an envelope that would have to be found outside the kernel.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP's sweep of a solid (sweepSolidWithin) carries a solid of revolution along a screw or a turn, or a prism along a planar motion, its outlines made of lines and circular arcs; a prism of a B-spline section under a screw is outside both, and its contact is an envelope that would have to be computed outside the kernel",
    ))
}

//! A rod grooved by an oblate-spheroid wheel carried along a screw.
//!
//! Refused: the kernel's sweep of a solid takes a solid of revolution along a screw, but its
//! meridian is made of lines and circular arcs only, and the spheroid's meridian is an ellipse.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP's sweep of a solid (sweepSolidWithin) carries a solid of revolution along a screw or a turn, or a prism along a planar motion, a meridian made of lines and circular arcs; the spheroid wheel's meridian is an ellipse, which that sweep does not take",
    ))
}

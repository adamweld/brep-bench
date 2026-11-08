//! A rod threaded by a bar carried along a path whose lead grows as it climbs.
//!
//! Refused: the kernel's sweep of a solid takes a motion of screws and turns each linear in one
//! parameter; a rise of 6s + 0.3s² is no such motion, and no screw carries the bar.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP's sweep of a solid (sweepSolidWithin) carries a solid of revolution along a screw or a turn, or a prism along a planar motion, each step linear in the motion's parameter; a rise of 6s + 0.3s² per turn is no such motion, and the bar is no solid of revolution",
    ))
}

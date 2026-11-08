//! A rod threaded by a drafted slot-section bar carried along a screw.
//!
//! Refused: the kernel's sweep of a solid takes a solid of revolution along a screw; the drafted
//! bar (two planes and two half cones about different axes) is no solid of revolution, nor a prism,
//! and its contact under the screw is an envelope no elementary composition gives.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP's sweep of a solid (sweepSolidWithin) carries a solid of revolution along a screw or a turn, or a prism along a planar motion; the drafted slot bar is neither a solid of revolution nor a prism, and the groove's walls under the screw are envelopes that would have to be computed outside the kernel",
    ))
}

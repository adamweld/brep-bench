//! The solid a regular tetrahedron fills as it is carried along a screw.
//!
//! Refused: the kernel's sweep of a solid takes a solid of revolution along a screw, or a prism
//! along a planar motion; a tetrahedron is neither, and its sweep's walls are envelopes no
//! elementary composition of the kernel's operations gives.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP's sweep of a solid (sweepSolidWithin) carries a solid of revolution along a screw or a turn, or a prism along a planar motion; a tetrahedron is neither, and the walls its edges sweep under the screw are envelopes that would have to be computed outside the kernel",
    ))
}

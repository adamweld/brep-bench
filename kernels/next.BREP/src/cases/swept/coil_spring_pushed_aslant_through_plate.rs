//! A plate holed by a coil spring (itself a ball swept along a helix) pushed aslant through it.
//!
//! Refused: the kernel's sweep of a solid takes a solid of revolution or a prism; the spring is
//! neither, and the hole is its shadow along the push, bounded by the coils' silhouettes, which
//! would have to be computed outside the kernel.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP's sweep of a solid (sweepSolidWithin) carries a solid of revolution along a screw or a turn, or a prism along a planar motion; the coil spring is neither, and the hole is its shadow along the push, bounded by the coils' silhouettes, which would have to be computed outside the kernel",
    ))
}

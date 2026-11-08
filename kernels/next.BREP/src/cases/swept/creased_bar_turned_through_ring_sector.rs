//! A ring sector grooved by a bar drafted both ways from its parting plane, turned past it.
//!
//! Refused: the kernel's sweep of a solid takes a solid of revolution along a turn, or a prism
//! along a planar motion; the creased, drafted bar leaning across the turn is neither, and its
//! sweep's walls are envelopes no elementary composition of the kernel's operations gives.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP's sweep of a solid (sweepSolidWithin) carries a solid of revolution along a screw or a turn, or a prism along a planar motion; the bar drafted both ways from a plane leaning across the turn is neither, and the walls it sweeps are envelopes that would have to be computed outside the kernel",
    ))
}

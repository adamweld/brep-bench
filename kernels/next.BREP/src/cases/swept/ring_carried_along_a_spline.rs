//! A block grooved by a ring carried along a spline by its moving frame, upright like a wheel.
//!
//! Refused: the kernel's sweep of a solid takes motions made of screws and turns about fixed axes;
//! a frame riding a Bézier curve is none, and the ring's sweep is an envelope that would have to be
//! computed outside the kernel.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP's sweep of a solid (sweepSolidWithin) carries a solid of revolution along a screw or a turn, or a prism along a planar motion, about fixed axes; a frame riding a Bézier curve is no such motion, and the ring's sweep is an envelope that would have to be computed outside the kernel",
    ))
}

//! A block grooved by an upright bull-nose mill run along a cubic Bézier curve that loops over
//! itself and turns tighter than the mill.
//!
//! Refused: next.BREP's sweep of a solid carries a solid of revolution only along a screw or a turn,
//! and a prism only in a planar motion; it has no sweep along a curve. The composition used for a
//! ball (the tube about the path with a ball at each end) does not carry over: the mill's section
//! swept square to the curve is the groove only where the curve turns no tighter than the mill,
//! and here it turns on a radius of about 1.79, under the mill's 2, so the swept section folds
//! through itself and the groove's inner wall is an envelope that would have to be computed outside
//! the kernel.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP has no sweep of a solid along a curve, and the curve turns tighter (radius about \
         1.79) than the mill (radius 2), so the mill's section swept along it folds through itself; \
         the groove's inner wall is an envelope the kernel does not compute",
    ))
}

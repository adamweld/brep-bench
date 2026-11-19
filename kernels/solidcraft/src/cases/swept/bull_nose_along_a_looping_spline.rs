//! A block, the box from (−10, −10, −8) to (30, 25, 0), with a groove cut into its top by the
//! bull-nose mill of `bull-nose-along-line-arc-line`, its upright axis's foot run along the cubic
//! Bézier curve in the top face through (0, 0, 0), (30, 20, 0), (−10, 20, 0) and (20, 0, 0), which
//! crosses itself and turns tighter than the mill.
//!
//! Refused. The mill's sweep is every point within R(z) of the curve at each height z, which is
//! the mill's section through its axis, square to the curve, swept along it, with a half mill at
//! each end. But where the curve turns tighter than R(z) that swept section folds through itself,
//! and where the curve crosses itself it passes through its own earlier part: the swept section
//! is not a solid. Trimming the fold away is finding where the mill cuts its own sweep, an
//! envelope.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or an \
         envelope: along this curve, which turns tighter than the mill and crosses itself, the \
         mill's section swept along it folds through itself, and the groove's walls are where \
         the mill cuts its own sweep"
        .into())
}

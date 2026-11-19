//! A block, the box from (−10, −10, −8) to (30, 25, 0), with a groove cut into its top by the
//! bull-nose mill of `bull-nose-along-line-arc-line`, its upright axis's foot run along the cubic
//! Bézier curve in the top face through (0, 0, 0), (30, 20, 0), (−10, 20, 0) and (20, 0, 0), which
//! crosses itself and turns tighter than the mill.
//!
//! solidcraft-kernel has no sweep of a solid, so the mill's sweep is built by the elementary
//! geometry of `bull-nose-along-line-arc-line`: every point within R(z) of the curve at each
//! height z, which is the mill's section through its axis square to the curve swept along it,
//! with the half mill beyond each end. That holds however the curve turns or crosses itself; where
//! it turns tighter than R(z) the swept section folds through itself, and where it crosses itself
//! the sweep passes through its own earlier part. The section is swept along the curve itself
//! (`PathSeg::Curve`, the sketch's Bézier), which keeps it upright along a flat curve.
use super::bull_nose_along_line_arc_line::{half_mill, section};
use crate::solids::*;
use solidcore_record::Seg2;
use solidcraft_kernel::{PathSeg, sweep};

pub fn build() -> R<Body> {
    let stock = block((-10.0, -10.0, -8.0), (30.0, 25.0, 0.0))?;
    let curve =
        Seg2::Cubic { p0: v2(0.0, 0.0), p1: v2(30.0, 20.0), p2: v2(-10.0, 20.0), p3: v2(20.0, 0.0) };
    let path = [PathSeg::Curve { plane: Plane::XY, seg: curve }];
    // the curve leaves (0, 0) along (3, 2) and reaches (20, 0) along (3, −2)
    let (leave, reach) = (v3(3.0, 2.0, 0.0) * (1.0 / 13f64.sqrt()), v3(3.0, -2.0, 0.0) * (1.0 / 13f64.sqrt()));
    // the section where the curve starts, square to it, w to its left
    let start = Plane { origin: Vec3::ZERO, x: Vec3::Z.cross(leave), y: Vec3::Z };
    let mut swept = sweep(&start, &section(), &path)?;
    for (foot, outward) in [(Vec3::ZERO, -leave), (v3(20.0, 0.0, 0.0), reach)] {
        swept = union(&swept, &half_mill(foot, outward)?)?;
    }
    cut(&stock, &swept)
}

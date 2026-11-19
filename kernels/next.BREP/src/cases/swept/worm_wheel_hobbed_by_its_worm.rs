//! A worm wheel's 30 teeth cut by its single-start worm (module 2, pitch radius 10, shafts square
//! and 40 apart): the worm turns by t about its axis (the line through (0, 40, 0) along x) while
//! the wheel turns by t/30 about z, t from 0 to 60π; the blank is the disk of radius 32 from
//! z = −6 to 6.
//!
//! Seen from the wheel the worm at t is turned by t about its axis and then by −t/30 about z, so
//! a point x of the wheel is in the worm at t where y = Rz(t/30)·x, carried back along the
//! worm's axis by the turn, lies in the thread. The thread is a screw's (turned by α it is the
//! infinite thread slid by −α), so that is: y + t·x̂ in the infinite thread, and y itself between
//! the worm's end planes. The whole motion comes back to its start after one turn of the wheel,
//! and each point of the blank comes under the worm once in it: its pass is the t near where the
//! turn brings it within the worm's tip radius.
//!
//! The 30 tooth spaces are one space turned by 12° steps (a worm turn moves the wheel by 12°
//! and leaves the worm as it was). One space is traced (`solids::gear`) in 13 slices z = −6.3 …
//! 6.3, seen from the point 40 from the axis at 84°, near the space's middle, and lofted into a
//! cutter; its 30 turned copies are cut from the blank.
use crate::solids::gear::{space, Generating, Slice};
use crate::solids::p;
use crate::solids::sweep::{moved, turn_z};
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, make_cylinder_brep, BooleanOperation, BrepSolid, Vec3};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

struct Worm;

const TIGHT: f64 = 1e-7;

impl Generating for Worm {
    fn margin(&self, x: Vec3, t: f64) -> (f64, u32) {
        let (c, s) = ((t / 30.0).cos(), (t / 30.0).sin());
        let y = p(c * x.x - s * x.y, s * x.x + c * x.y, x.z);
        let (a, b) = (y.y - 40.0, y.z);
        let r = a.hypot(b);
        let theta = b.atan2(a);
        let xi = (y.x + t - theta + PI).rem_euclid(TAU) - PI;
        let h = FRAC_PI_2 - (r - 10.0) * 20f64.to_radians().tan();
        let slack = [h - xi, h + xi, 12.5 - r, r - 7.5, 20.0 - y.x.abs()];
        let least = slack.iter().copied().fold(f64::INFINITY, f64::min);
        let bind = slack.iter().enumerate().filter(|(_, &v)| v - least <= TIGHT).fold(0u32, |m, (i, _)| m | 1 << i);
        (least, bind)
    }

    fn window(&self, x: Vec3) -> Option<(f64, f64, f64)> {
        let rho = x.x.hypot(x.y);
        if x.z.abs() >= 12.5 {
            return None;
        }
        // Where the turned point comes within the worm's tip radius: y ≥ 40 − √(12.5² − z²).
        let v = (40.0 - (12.5f64.powi(2) - x.z * x.z).sqrt()) / rho;
        if v >= 1.0 {
            return None;
        }
        let phi = x.y.atan2(x.x);
        let (lo, hi) = (v.asin() - 0.01, PI - v.asin() + 0.01);
        Some((30.0 * (lo - phi), 30.0 * (hi - phi), 0.1))
    }
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let blank = make_cylinder_brep(p(0.0, 0.0, -6.0), p(0.0, 0.0, 1.0), 32.0, 12.0)?;
    let middle = 84f64.to_radians();
    let q = [40.0 * middle.cos(), 40.0 * middle.sin()];
    let toward = [-middle.cos(), -middle.sin()];
    let r_top = 32.2;
    // The rays' angles from `toward` reaching the points of r = r_top at 78.3° and 89.7°: on the
    // teeth's tops beside the space at every slice (the space keeps within 78.5°–89.5° even at
    // r = 32.5), so that no two turned copies touch.
    let angle_to = |deg: f64| {
        let a = deg.to_radians();
        let d = [r_top * a.cos() - q[0], r_top * a.sin() - q[1]];
        (toward[0] * d[1] - toward[1] * d[0]).atan2(toward[0] * d[0] + toward[1] * d[1])
    };
    let (from, to) = (angle_to(78.3).min(angle_to(89.7)), angle_to(78.3).max(angle_to(89.7)));
    // 13 slices from z = −6.3 to 6.3, just past the blank's faces: by z = ±6.5 the thread's
    // edge tracks have left the space's boundary and the pieces change.
    let slices: Vec<Slice> = (0..=12)
        .map(|k| Slice { z: -6.3 + 1.05 * k as f64, q, toward, from, to, r_top, reach: 20.0 })
        .collect();
    let space = space(&Worm, &slices, 120, 25)?;
    let mut operands = vec![blank];
    for k in 0..30 {
        operands.push(moved(&space, turn_z(12f64.to_radians() * k as f64, p(0.0, 0.0, 0.0))?)?);
    }
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

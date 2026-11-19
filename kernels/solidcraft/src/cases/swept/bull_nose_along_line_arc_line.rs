//! A block, the box from (−10, −10, −10) to (40, 40, 0), with a groove cut into its top by a
//! bull-nose mill whose upright axis runs along x from (0, 0) to (20, 0), a quarter turn about the
//! vertical line through (20, 10) to (30, 10), and along y to (30, 30). The mill is a cylinder of
//! radius 2 from z = −1.5 to z = 5, its bottom corner rounded to radius 0.5.
//!
//! solidcraft-kernel has no sweep of a solid, so the mill's sweep is built by elementary geometry.
//! The mill is a solid of revolution about its upright axis, so carried rigidly or slid it fills
//! the same places, and at each height z it is a disc of radius R(z) whose centre runs along the
//! path: it fills every point within R(z) of the path. A point within R(z) of the path whose
//! nearest point on it is inside the path lies on the path's normal there, within R(z); one whose
//! nearest point is an end lies in the half of the disc there beyond that end. So the sweep is the
//! mill's section through its axis, square to the path, swept along the path (the kernel's
//! `sweep`, which turns it with the path, keeping it upright along this flat path) with the half
//! mill beyond each end.
use crate::solids::*;
use solidcore_record::Seg2;
use solidcraft_kernel::{PathSeg, sweep};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// The mill's section through its axis, in (w, z): w across the axis, z up from the block's top.
pub fn section() -> Region2 {
    let segs = vec![
        Seg2::Line { a: v2(-1.5, -1.5), b: v2(1.5, -1.5) },
        Seg2::Arc { center: v2(1.5, -1.0), radius: 0.5, start: -FRAC_PI_2, sweep: FRAC_PI_2 },
        Seg2::Line { a: v2(2.0, -1.0), b: v2(2.0, 5.0) },
        Seg2::Line { a: v2(2.0, 5.0), b: v2(-2.0, 5.0) },
        Seg2::Line { a: v2(-2.0, 5.0), b: v2(-2.0, -1.0) },
        Seg2::Arc { center: v2(-1.5, -1.0), radius: 0.5, start: PI, sweep: FRAC_PI_2 },
    ];
    Region2 { outer: Loop2 { segs }, holes: vec![] }
}

/// The half of the mill whose axis is the vertical line through `foot` (on the block's top) on
/// the side `outward` points to: its half section revolved about the axis, then halved.
pub fn half_mill(foot: Vec3, outward: Vec3) -> R<Body> {
    let half = Region2 {
        outer: Loop2 {
            segs: vec![
                Seg2::Line { a: v2(0.0, -1.5), b: v2(1.5, -1.5) },
                Seg2::Arc { center: v2(1.5, -1.0), radius: 0.5, start: -FRAC_PI_2, sweep: FRAC_PI_2 },
                Seg2::Line { a: v2(2.0, -1.0), b: v2(2.0, 5.0) },
                Seg2::Line { a: v2(2.0, 5.0), b: v2(0.0, 5.0) },
                Seg2::Line { a: v2(0.0, 5.0), b: v2(0.0, -1.5) },
            ],
        },
        holes: vec![],
    };
    let plane = Plane { origin: foot, x: Vec3::X, y: Vec3::Z };
    let mill = one(solidcraft_kernel::revolve(&plane, &[half], Vec2::ZERO, Vec2::Y, TAU)?)?;
    let cap = Plane::from_normal(foot, outward)
        .ok_or_else(|| KernelError::Invalid("the half mill's direction".into()))?;
    solidcraft_kernel::half_space(&mill, &cap, true)?
        .0
        .ok_or_else(|| KernelError::Failed("the half mill is empty".into()))
}

pub fn build() -> R<Body> {
    let stock = block((-10.0, -10.0, -10.0), (40.0, 40.0, 0.0))?;
    let path = [
        PathSeg::Line { a: v3(0.0, 0.0, 0.0), b: v3(20.0, 0.0, 0.0) },
        // a quarter turn about +z, from −y toward +x of (20, 10)
        PathSeg::Arc { a: v3(20.0, 0.0, 0.0), center: v3(20.0, 10.0, 0.0), axis: Vec3::Z, angle: FRAC_PI_2 },
        PathSeg::Line { a: v3(30.0, 10.0, 0.0), b: v3(30.0, 30.0, 0.0) },
    ];
    // the section where the path starts, square to it (it leaves along +x), w along +y
    let start = Plane { origin: Vec3::ZERO, x: Vec3::Y, y: Vec3::Z };
    let mut swept = sweep(&start, &section(), &path)?;
    // the path leaves (0, 0) along +x and reaches (30, 30) along +y
    for (foot, outward) in [(v3(0.0, 0.0, 0.0), -Vec3::X), (v3(30.0, 30.0, 0.0), Vec3::Y)] {
        swept = union(&swept, &half_mill(foot, outward)?)?;
    }
    cut(&stock, &swept)
}

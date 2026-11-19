//! The solid a ring fills as it slides without turning along x by 15: the torus about the z axis
//! centred on the origin, the circle through its tube's centres of radius 6, its tube of radius 2.
//!
//! solidcraft-kernel has no sweep of a solid, so the ring's sweep is built by elementary geometry,
//! level by level. At height z the ring is an annulus, radii 6 ∓ √(4 − z²). A point its outer
//! disc slides over is missed by the annulus only if the point's whole path back, 15 long, lies
//! in the hole, at most 12 wide: never. So the ring sweeps what the ring with its hole filled
//! sweeps. That filled ring is convex, the revolve of its half section. Split by the plane x = 0
//! through its axis, square to the slide, its back half leads with its flat cut face only (its
//! round faces back, its flat top and bottom move in their own planes), so it sweeps itself and
//! that section slid 15; its front half likewise sweeps the section and itself where it stops.
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// The half of the ring centred on (`x`, 0, 0), its hole filled, on the side `outward` points to.
fn half_filled(x: f64, outward: Vec3) -> R<Body> {
    // the half section in (r, z): out to the tube's outer half
    let segs = vec![
        Seg2::Line { a: v2(0.0, -2.0), b: v2(6.0, -2.0) },
        Seg2::Arc { center: v2(6.0, 0.0), radius: 2.0, start: -FRAC_PI_2, sweep: PI },
        Seg2::Line { a: v2(6.0, 2.0), b: v2(0.0, 2.0) },
        Seg2::Line { a: v2(0.0, 2.0), b: v2(0.0, -2.0) },
    ];
    let plane = Plane { origin: v3(x, 0.0, 0.0), x: Vec3::X, y: Vec3::Z };
    let section = Region2 { outer: Loop2 { segs }, holes: vec![] };
    let filled = one(solidcraft_kernel::revolve(&plane, &[section], Vec2::ZERO, Vec2::Y, TAU)?)?;
    let cap = Plane::from_normal(plane.origin, outward)
        .ok_or_else(|| KernelError::Invalid("the half ring's direction".into()))?;
    solidcraft_kernel::half_space(&filled, &cap, true)?
        .0
        .ok_or_else(|| KernelError::Failed("the half ring is empty".into()))
}

/// What the ring with its hole filled sweeps as it slides along x by `slide`: its back half where
/// it starts, its section through the axis on x = 0 extruded along x, and its front half where it
/// stops.
pub fn filled_slid(slide: f64) -> R<Body> {
    // the section in (y, z): within 2 of the segment from (−6, 0) to (6, 0), the flat between
    let segs = vec![
        Seg2::Line { a: v2(-6.0, -2.0), b: v2(6.0, -2.0) },
        Seg2::Arc { center: v2(6.0, 0.0), radius: 2.0, start: -FRAC_PI_2, sweep: PI },
        Seg2::Line { a: v2(6.0, 2.0), b: v2(-6.0, 2.0) },
        Seg2::Arc { center: v2(-6.0, 0.0), radius: 2.0, start: FRAC_PI_2, sweep: PI },
    ];
    let plane = Plane { origin: Vec3::ZERO, x: Vec3::Y, y: Vec3::Z };
    let mut swept = extrude(&plane, Region2 { outer: Loop2 { segs }, holes: vec![] }, 0.0, slide)?;
    swept = union(&swept, &half_filled(0.0, -Vec3::X)?)?;
    union(&swept, &half_filled(slide, Vec3::X)?)
}

pub fn build() -> R<Body> {
    filled_slid(15.0)
}

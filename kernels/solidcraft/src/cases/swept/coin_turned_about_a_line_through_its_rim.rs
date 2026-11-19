//! The solid a thick disc fills as it is turned 90° about the z axis, right-handed: the solid
//! cylinder of radius 2 about the line through (2, 0, 0) along y, from y = 0 to 0.5, whose rim in
//! the plane y = 0 passes through the origin, on the z axis.
//!
//! solidcore-ops has no sweep of a solid, so the turn is built by elementary geometry, from the
//! kernel's revolves of flat regions about the z axis. At height z the disc is the rectangle
//! 2 − c ≤ x ≤ 2 + c, 0 ≤ y ≤ 0.5, c = √(4 − z²), which meets each circle about the z axis in one
//! arc (going round, x falls and y rises), a few degrees wide. Turned by 90°, a solid that meets
//! each circle in one arc passes over that arc where it starts, the arc where it stops, and
//! between them the 90° on from any one point of the arc; so the sweep is the disc where it stops
//! with, revolved through the turn, flat regions of the disc that between them meet every circle
//! the disc meets, and with what of the disc where it starts lies behind them. The half of its
//! face in the plane y = 0 nearer the axis (x ≤ 2) meets the circles out from 2 − c to 2, which it
//! meets at 0°, the disc's first point on them, so nothing of the disc there lies behind it; its
//! section by the plane x = 2 (the rectangle 0 ≤ y ≤ 0.5, |z| ≤ 2) meets those from 2 to
//! √(2² + 0.5²), and the half of its face in the plane y = 0.5 beyond that plane those on out to
//! its far corners, √((2 + c)² + 0.5²). What lies behind those two on their circles is the
//! disc's half beyond the plane x = 2. Each of the three regions meets each circle once, so turns
//! without folding over itself.
use crate::bench::{arc, common, fuse_all, line, polygon, rotated, translated};
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::{make_box, make_cylinder, Body};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<Body, Refusal> {
    let (z, turn) = ([0.0, 0.0, 1.0], FRAC_PI_2);
    // the kernel's cylinder stands on +z: turned a quarter turn back about x it lies along +y
    let disc = || -> Result<Body, Refusal> {
        let upright = make_cylinder([0.0; 3], 2.0, 0.5)?;
        translated(rotated(upright, [0.0; 3], [1.0, 0.0, 0.0], -90.0)?, [2.0, 0.0, 0.0])
    };
    // about +y, angle 0 along +x, so counter-clockwise runs from −z through −x (the origin) to +z
    let near_half = [
        arc([2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0], 2.0, FRAC_PI_2, 3.0 * FRAC_PI_2),
        line([2.0, 0.0, 2.0], [2.0, 0.0, -2.0]),
    ];
    let middle = polygon(&[[2.0, 0.0, -2.0], [2.0, 0.5, -2.0], [2.0, 0.5, 2.0], [2.0, 0.0, 2.0]]);
    // about −y, angle 0 along +x, so counter-clockwise runs from −z through +x up to +z
    let far_half = [
        arc([2.0, 0.5, 0.0], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 2.0, -FRAC_PI_2, FRAC_PI_2),
        line([2.0, 0.5, 2.0], [2.0, 0.5, -2.0]),
    ];
    let behind = common(disc()?, make_box([2.0, -1.0, -3.0], [3.0, 3.0, 6.0])?)?;
    let pieces = [
        revolve(&near_half, [0.0; 3], z, turn, true)?,
        revolve(&middle, [0.0; 3], z, turn, true)?,
        revolve(&far_half, [0.0; 3], z, turn, true)?,
        behind,
        rotated(disc()?, [0.0; 3], z, 90.0)?,
    ];
    fuse_all(pieces)
}

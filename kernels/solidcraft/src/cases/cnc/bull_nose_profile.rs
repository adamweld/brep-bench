//! A plate, the box from (0, 0, −16) to (120, 120, 0), with a groove milled round a six-lobed
//! flower by a bull-nose mill of radius 4, its end corner rounded to 1.5, 25 long, upright:
//! plunged at (102, 60) and run anticlockwise round the flower (lobes of radius 12 about points
//! 30 from (60, 60), valleys tangent to them about points 40 from it) at z = −4, −8 and −10, then
//! risen.
//!
//! solidcraft-kernel has no sweep of a solid, so each lap's sweep is built by elementary
//! geometry, as `flat_end_mill_profile` builds its laps: the mill's section through its axis
//! (8 wide, its two bottom corners rounded to 1.5) carried round the closed path square to it.
//! The flower turns no tighter than radius 8.53, more than the mill's 4. The plunges and the
//! rise, at (102, 60), a point of the flower, lie within the laps' grooves.
use crate::cases::cnc::flat_end_mill_profile::round_the_path;
use crate::solids::*;
use solidcore_record::Seg2;
use solidcraft_kernel::PathSeg;
use std::f64::consts::{FRAC_PI_2, TAU};

/// The flower at height `z`, from (102, 60) anticlockwise and back: thirteen arcs, the lobe at
/// 0° in two.
fn flower(z: f64) -> Vec<PathSeg> {
    let d = (2500.0 - 1200.0 * 3f64.sqrt()).sqrt();
    let about = |r: f64, degrees: f64| {
        let a = degrees.to_radians();
        v2(60.0 + r * a.cos(), 60.0 + r * a.sin())
    };
    let lobe = |k: usize| about(30.0, 60.0 * k as f64);
    let valley = |k: usize| about(40.0, 30.0 + 60.0 * k as f64);
    // the way from one centre to another: where circles about them touch
    let toward = |a: Vec2, b: Vec2| (b.y - a.y).atan2(b.x - a.x);
    let anticlockwise = |c: Vec2, r: f64, from: f64, to: f64| Seg2::Arc {
        center: c,
        radius: r,
        start: from,
        sweep: (to - from).rem_euclid(TAU),
    };
    let clockwise = |c: Vec2, r: f64, from: f64, to: f64| Seg2::Arc {
        center: c,
        radius: r,
        start: from,
        sweep: -(from - to).rem_euclid(TAU),
    };
    let mut segs = Vec::new();
    for k in 0..6 {
        let (l, v, next) = (lobe(k), valley(k), lobe((k + 1) % 6));
        // along the lobe from its outermost point (or where the last valley left it) to where it
        // touches its valley; round the valley to the next lobe
        let from = if k == 0 { 0.0 } else { toward(l, valley(k - 1)) };
        segs.push(anticlockwise(l, 12.0, from, toward(l, v)));
        segs.push(clockwise(v, d - 12.0, toward(v, l), toward(v, next)));
    }
    segs.push(anticlockwise(lobe(0), 12.0, toward(lobe(0), valley(5)), 0.0));
    let level = Plane { origin: v3(0.0, 0.0, z), x: Vec3::X, y: Vec3::Y };
    segs.iter().map(|s| PathSeg::of_sketch(&level, s)).collect()
}

/// The mill's section through its axis, across and up from its tip: its flat end 5 wide, its
/// corners rounded to 1.5, its sides 8 apart, 25 high.
fn section() -> Region2 {
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    let segs = vec![
        line((-2.5, 0.0), (2.5, 0.0)),
        Seg2::Arc { center: v2(2.5, 1.5), radius: 1.5, start: -FRAC_PI_2, sweep: FRAC_PI_2 },
        line((4.0, 1.5), (4.0, 25.0)),
        line((4.0, 25.0), (-4.0, 25.0)),
        line((-4.0, 25.0), (-4.0, 1.5)),
        Seg2::Arc { center: v2(-2.5, 1.5), radius: 1.5, start: std::f64::consts::PI, sweep: FRAC_PI_2 },
    ];
    Region2 { outer: Loop2 { segs }, holes: vec![] }
}

pub fn build() -> R<Body> {
    let mut plate = block((0.0, 0.0, -16.0), (120.0, 120.0, 0.0))?;
    for z in [-4.0, -8.0, -10.0] {
        plate = cut(&plate, &round_the_path(section(), &flower(z))?)?;
    }
    Ok(plate)
}

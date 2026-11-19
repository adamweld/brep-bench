//! A dome machined by a five-axis program of four cutters. The blank is the cylinder of radius 50
//! about the z axis from z = 0 to 45; the dome aimed at, the ball of radius 40 about the origin.
//! A bull-nose mill (radius 8, corner 2, 40 long) roughs it upright in eighteen level circles; a
//! ball-end mill of radius 4, 40 long, held square to the dome, finishes it along 180 meridians
//! (latitude 3° to 75°, 2° apart), round 15 circles near the pole and with a touch at the pole; a
//! flat end mill of radius 5 faces twelve seats at latitude 40°, and a drill of radius 2.5 with a
//! 90° point drills each, both square to the dome.
//!
//! solidcraft-kernel has no sweep of a solid, so each move's sweep is built by elementary
//! geometry. Every cutter is a solid of revolution, and every turn here is about a line that
//! meets the cutter's axis or runs beside it (the z axis, or a level line through the origin):
//! the plane through both lines is one the cutter is symmetric about, and each point's path is
//! square to it, so what the turn sweeps is the cutter's section in that plane revolved about
//! the line, with the cutter where it starts and where it ends; a whole turn is the section
//! revolved alone. A cutter slid along its own axis fills itself where its tip is deepest, its
//! side carried out to where it ends. The moves made at 90 or 70 from the origin, or at z = 50
//! with the axis upright, stand clear of the blank and cut nothing, and the slides in and out
//! of a whole turn lie within what it sweeps, or beyond the blank.
//!
//! The program repeats itself round the z axis, and the dome is built as it does: one sector
//! machined, and patterned round. The roughing, the circles round the pole and the touch at the
//! pole are whole turns about the z axis, the same at every longitude. Each meridian pass is the
//! last turned 2° on, and is its own mirror image in its meridian plane; on any circle about the
//! z axis within the blank what it covers is an arc with its middle on its meridian (the mill
//! reaches out from the z axis along its meridian, no wider than 8). So within 1° of a pass's
//! meridian the passes together cover what that pass covers alone. The 1° from the first pass's
//! meridian is machined by the whole turns and that pass, mirrored to make the pass's 2°; fifteen
//! of those make the 30° from −1° to 29°, which holds one seat and hole, at 15°; and twelve of
//! those make the dome.
use crate::solids::*;
use solidcore_record::Seg2;
use solidcraft_kernel::planar_regions_boolean;
use std::f64::consts::{FRAC_PI_2, PI};

/// The dome's outward normal at a latitude and a longitude (degrees).
fn normal(lat: f64, lon: f64) -> Vec3 {
    let (l, p) = (lat.to_radians(), lon.to_radians());
    v3(l.cos() * p.cos(), l.cos() * p.sin(), l.sin())
}

/// The ball-end mill's section through its axis, in the distance along the axis and across it:
/// its tip at `from`, the half disc of radius 4 below the side, out to `to`.
fn ball_end(from: f64, to: f64) -> Region2 {
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    let c = from + 4.0;
    let segs = vec![
        line((c, -4.0), (to, -4.0)),
        line((to, -4.0), (to, 4.0)),
        line((to, 4.0), (c, 4.0)),
        Seg2::Arc { center: v2(c, 0.0), radius: 4.0, start: FRAC_PI_2, sweep: PI },
    ];
    Region2 { outer: Loop2 { segs }, holes: vec![] }
}

/// Half of [`ball_end`], on one side of the axis.
fn half_ball_end(from: f64, to: f64) -> Region2 {
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    let c = from + 4.0;
    let segs = vec![
        line((from, 0.0), (to, 0.0)),
        line((to, 0.0), (to, 4.0)),
        line((to, 4.0), (c, 4.0)),
        Seg2::Arc { center: v2(c, 0.0), radius: 4.0, start: FRAC_PI_2, sweep: FRAC_PI_2 },
    ];
    Region2 { outer: Loop2 { segs }, holes: vec![] }
}

/// [`ball_end`] from 40 to 80 in the plane y = 0 (x and z), its axis at latitude `lat` from +x
/// toward +z.
fn leaning(lat: f64) -> Region2 {
    let a = lat.to_radians();
    let turn = |p: Vec2| v2(p.x * a.cos() - p.y * a.sin(), p.x * a.sin() + p.y * a.cos());
    let segs = ball_end(40.0, 80.0)
        .outer
        .segs
        .iter()
        .map(|s| match *s {
            Seg2::Line { a: p, b: q } => Seg2::Line { a: turn(p), b: turn(q) },
            Seg2::Arc { center, radius, start, sweep } => Seg2::Arc { center: turn(center), radius, start: start + a, sweep },
            _ => unreachable!("the section is lines and an arc"),
        })
        .collect();
    Region2 { outer: Loop2 { segs }, holes: vec![] }
}

/// The plane through the z axis and the meridian at longitude 0 turned about the level line
/// along −y: through the origin, its sketch x along the normal at `lat`, its sketch y along y,
/// its normal the way the latitude grows.
fn across_meridian(lat: f64) -> Plane {
    Plane { origin: Vec3::ZERO, x: normal(lat, 0.0), y: Vec3::Y }
}

fn turned(body: &Body, axis: Vec3, degrees: f64) -> R<Body> {
    solidcraft_kernel::transform(body, Vec3::ZERO, Vec3::ZERO, axis, degrees.to_radians())
}

/// `part` turned about the z axis `count` times by `step` degrees, joined.
fn round_the_axis(part: &Body, count: usize, step: f64) -> R<Body> {
    let copies = (0..count).map(|i| turned(part, Vec3::Z, step * i as f64)).collect::<R<Vec<_>>>()?;
    solidcraft_kernel::union_all(&copies.iter().collect::<Vec<_>>())
}

pub fn build() -> R<Body> {
    // the blank from the first meridian's plane, y = 0, 1° on
    let mut part = revolve_z(rect(0.0, 0.0, 50.0, 45.0), 1.0)?;
    let mut tools = Vec::new();
    // 1. Roughing: each circle the bull-nose mill's section through its axis turned once about
    // the z axis (its plunge and rise, at a point of the circle, within it)
    for z in [41.0f64, 36.0, 31.0, 26.0, 21.0, 16.0, 11.0, 6.0] {
        let rho = (40.0f64 * 40.0 - z * z).max(0.0).sqrt();
        let mut r = rho + 9.0;
        while r < 58.0 {
            let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
            let segs = vec![
                line((r - 6.0, z), (r + 6.0, z)),
                Seg2::Arc { center: v2(r + 6.0, z + 2.0), radius: 2.0, start: -FRAC_PI_2, sweep: FRAC_PI_2 },
                line((r + 8.0, z + 2.0), (r + 8.0, z + 40.0)),
                line((r + 8.0, z + 40.0), (r - 8.0, z + 40.0)),
                line((r - 8.0, z + 40.0), (r - 8.0, z + 2.0)),
                Seg2::Arc { center: v2(r - 6.0, z + 2.0), radius: 2.0, start: PI, sweep: FRAC_PI_2 },
            ];
            tools.push(revolve_z(Region2 { outer: Loop2 { segs }, holes: vec![] }, 360.0)?);
            r += 12.0;
        }
    }
    // 2. Finishing along the meridian at longitude 0: slid in along its axis at latitude 3°,
    // turned 72° about the level line along −y up to 75°, slid out along its axis. The turn
    // sweeps the section in the plane through that line and the axis, revolved; at each end the
    // mill, slid out to 130 from the origin, its half beyond the turn's section.
    let turn = solidcraft_kernel::revolve(&across_meridian(3.0), &[ball_end(40.0, 80.0)], Vec2::ZERO, Vec2::Y, 72f64.to_radians())?;
    tools.push(one(turn)?);
    for (lat, ahead) in [(3.0, -PI), (75.0, PI)] {
        let half = solidcraft_kernel::revolve(&across_meridian(lat), &[half_ball_end(40.0, 130.0)], Vec2::ZERO, Vec2::X, ahead)?;
        tools.push(one(half)?);
    }
    // round the pole: the leaning mill turned once about the z axis, its section in the plane
    // y = 0 revolved; where it reaches past the axis, the part beyond folded back onto the near
    // side (the section and its mirror image in the axis, on the near side)
    for lat in 75..90 {
        let both = planar_regions_boolean(&[leaning(lat as f64)], &[leaning(180.0 - lat as f64)], BoolOp::Union, 1e-9)?;
        let near = planar_regions_boolean(&both, &[rect(0.0, 0.0, 200.0, 200.0)], BoolOp::Intersect, 1e-9)?;
        for section in near {
            tools.push(revolve_z(section, 360.0)?);
        }
    }
    // the touch at the pole: the mill upright, from its tip at 40 out to 130
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    let pole = vec![
        Seg2::Arc { center: v2(0.0, 44.0), radius: 4.0, start: -FRAC_PI_2, sweep: FRAC_PI_2 },
        line((4.0, 44.0), (4.0, 130.0)),
        line((4.0, 130.0), (0.0, 130.0)),
        line((0.0, 130.0), (0.0, 40.0)),
    ];
    tools.push(revolve_z(Region2 { outer: Loop2 { segs: pole }, holes: vec![] }, 360.0)?);
    for tool in &tools {
        part = cut(&part, tool)?;
    }
    // the pass's own 2°, and fifteen passes' 30°, from −1° to 29°
    let mirror = solidcraft_kernel::transform_matrix(
        &part,
        [[1.0, 0.0, 0.0, 0.0], [0.0, -1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
    )?;
    let slice = union(&part, &mirror)?;
    let mut sector = round_the_axis(&slice, 15, 2.0)?;
    // 3. The seat and 4. the hole at longitude 15°, each cutter slid in along the normal and back
    let n = normal(40.0, 15.0);
    let seat = solidcraft_kernel::cylinder(n * 38.5, n, 5.0, 110.0 - 38.5)?;
    sector = cut(&sector, &seat)?;
    let drill = revolve_z(polygon(&[(0.0, 0.0), (2.5, 2.5), (2.5, 80.0), (0.0, 80.0)]), 360.0)?;
    let drill = turned(&drill, Vec3::Y, 50.0)?;
    let drill = solidcraft_kernel::transform(&drill, n * 30.0, Vec3::ZERO, Vec3::Z, 15f64.to_radians())?;
    sector = cut(&sector, &drill)?;
    round_the_axis(&sector, 12, 30.0)
}

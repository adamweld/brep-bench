//! A plate, the box from (0, 0, −20) to (160, 110, 0), machined by a 2.5-axis program of three
//! upright tools. A flat end mill of radius 5, 30 long, clears a pocket round an island at
//! z = −4, −8 and −12: raster passes along y = 26, 30, …, 82 across the region its tip keeps to
//! (inside the wall path, the rectangle from (25, 25) to (135, 85) rounded to 7, and outside the
//! island path, the circle of radius 20 about (60, 55)), then a lap round each path. A flat end
//! mill of radius 3 bores the island's top in two circles of radius 6, at z = −3 and −6. A drill
//! of radius 4 with a 90° point pecks four holes, its tip down to z = −14.
//!
//! solidcraft-kernel has no sweep of a solid, so each move's sweep is built by elementary
//! geometry. An upright cylinder slid along a level line fills its shadow along z, the slot within
//! its radius of the line, from its tip up; carried round a level path, its section through its
//! axis carried round the path square to it (as in `flat_end_mill_profile`); turned once about an
//! upright line, its section in the plane through that line revolved. A tool slid along its own
//! axis fills itself where its tip is lowest, its side carried up to where its top is highest.
//! The moves at z = 5 are above the plate and cut nothing. Each plunge and rise of a pass or a lap
//! lies within the slot or groove it starts or ends, within the plate, and each peck of a hole
//! within the deepest.
use crate::cases::cnc::flat_end_mill_profile::{round_the_path, rounded_loop};
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// Where the line at `y` crosses the region the pocketing mill's tip keeps to: inside the wall
/// path and outside the island path.
fn intervals(y: f64) -> Vec<(f64, f64)> {
    // the wall path's rounded corners, about (32, 32), (128, 32), (128, 78) and (32, 78)
    let off = if y < 32.0 { 32.0 - y } else if y > 78.0 { y - 78.0 } else { 0.0 };
    let left = 32.0 - (49.0 - off * off).sqrt();
    let right = 160.0 - left;
    let dy = (y - 55.0).abs();
    if dy < 20.0 {
        let half = (400.0 - dy * dy).sqrt();
        vec![(left, 60.0 - half), (60.0 + half, right)]
    } else {
        vec![(left, right)]
    }
}

/// What an upright flat end mill of radius `r` and `length` fills slid from (`a`, `y`) to
/// (`b`, `y`) at the height `z` of its tip: the slot within `r` of that segment, from `z` up.
fn slot(a: f64, b: f64, y: f64, r: f64, z: f64, length: f64) -> R<Body> {
    let segs = vec![
        Seg2::Line { a: v2(a, y - r), b: v2(b, y - r) },
        Seg2::Arc { center: v2(b, y), radius: r, start: -FRAC_PI_2, sweep: PI },
        Seg2::Line { a: v2(b, y + r), b: v2(a, y + r) },
        Seg2::Arc { center: v2(a, y), radius: r, start: FRAC_PI_2, sweep: PI },
    ];
    let level = Plane { origin: v3(0.0, 0.0, z), x: Vec3::X, y: Vec3::Y };
    extrude(&level, Region2 { outer: Loop2 { segs }, holes: vec![] }, 0.0, length)
}

/// What an upright mill fills turned once about the upright line through `c`: its section
/// (`section`, in the distance out from the line and the height) revolved.
fn round_the_line(c: (f64, f64), section: Region2) -> R<Body> {
    let plane = Plane { origin: v3(c.0, c.1, 0.0), x: Vec3::X, y: Vec3::Z };
    one(solidcraft_kernel::revolve(&plane, &[section], Vec2::ZERO, Vec2::Y, TAU)?)
}

pub fn build() -> R<Body> {
    let plate = block((0.0, 0.0, -20.0), (160.0, 110.0, 0.0))?;
    let mut tools = Vec::new();
    // the pocket: at each depth the raster passes, the lap round the wall path and the lap round
    // the island path
    let wall = [(25.0, 25.0), (135.0, 25.0), (135.0, 85.0), (25.0, 85.0)];
    for z in [-4.0, -8.0, -12.0] {
        for k in 0..15 {
            let y = 26.0 + 4.0 * k as f64;
            for (a, b) in intervals(y) {
                tools.push(slot(a, b, y, 5.0, z, 30.0)?);
            }
        }
        tools.push(round_the_path(rect(-5.0, 0.0, 5.0, 30.0), &rounded_loop(&wall, 7.0, z))?);
        tools.push(round_the_line((60.0, 55.0), rect(15.0, z, 25.0, z + 30.0))?);
    }
    // the bore: at each depth out from the centre to radius 6, a circle about it, and back
    for z in [-3.0, -6.0] {
        tools.push(slot(60.0, 66.0, 55.0, 3.0, z, 30.0)?);
        tools.push(round_the_line((60.0, 55.0), rect(3.0, z, 9.0, z + 30.0))?);
    }
    // the holes: the drill where its tip is lowest, z = −14, its side up to its top where it
    // rises to, 30 above z = 5
    for (x, y) in [(8.0, 8.0), (152.0, 8.0), (152.0, 102.0), (8.0, 102.0)] {
        let drill = polygon(&[(0.0, -14.0), (4.0, -10.0), (4.0, 35.0), (0.0, 35.0)]);
        tools.push(round_the_line((x, y), drill)?);
    }
    let tools: Vec<&Body> = tools.iter().collect();
    cut(&plate, &solidcraft_kernel::union_all(&tools)?)
}

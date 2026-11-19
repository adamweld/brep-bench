//! A plate, the box from (0, 0, −15) to (120, 90, 0), with a groove milled round a cross by a
//! flat end mill of radius 3, 20 long, upright: plunged at (60, 10) and run anticlockwise round
//! the cross's outline (every corner rounded to 6) at z = −3, −6 and −9, then risen.
//!
//! solidcraft-kernel has no sweep of a solid, so each lap's sweep is built by elementary
//! geometry. An upright cylinder carried round a level path fills, at each height it spans, the
//! ground within its radius of the path: its section through its axis square to the path (6
//! wide, 20 high) carried round the path square to it, the kernel's sweep of a profile. The path
//! is closed, and turns no tighter than radius 6, more than the mill's 3, so the section folds
//! nowhere and the mill's two halves at the lap's ends lie in its groove. The plunges and the
//! rise, at (60, 10), a point of the path, lie within the laps' grooves too.
use crate::solids::*;
use solidcore_record::Seg2;
use solidcraft_kernel::{PathSeg, sweep};

/// The cross, its corners anticlockwise from (45, 10).
const CROSS: [(f64, f64); 12] = [
    (45.0, 10.0), (75.0, 10.0), (75.0, 30.0), (100.0, 30.0), (100.0, 60.0), (75.0, 60.0),
    (75.0, 80.0), (45.0, 80.0), (45.0, 60.0), (20.0, 60.0), (20.0, 30.0), (45.0, 30.0),
];

/// The closed path round the polygon `corners` (anticlockwise) at height `z`, every corner
/// rounded to `radius` by an arc tangent to both its sides (anticlockwise round an outside
/// corner, clockwise round an inside one): from the middle of the first side round and back to
/// it.
pub fn rounded_loop(corners: &[(f64, f64)], radius: f64, z: f64) -> Vec<PathSeg> {
    let n = corners.len();
    let at = |k: usize| v2(corners[k % n].0, corners[k % n].1);
    let dir = |k: usize| {
        let d = at(k + 1) - at(k);
        d * (1.0 / d.len())
    };
    let level = Plane { origin: v3(0.0, 0.0, z), x: Vec3::X, y: Vec3::Y };
    let mut segs = Vec::new();
    let middle = (at(0) + at(1)) * 0.5;
    let mut from = middle;
    for k in 1..=n {
        let (c, a, b) = (at(k), dir(k - 1), dir(k));
        // a left turn's arc is about the point `radius` to the left of both sides, a right
        // turn's to their right
        let side = (a.x * b.y - a.y * b.x).signum();
        let (leave, join) = (c - a * radius, c + b * radius);
        let centre = leave + v2(-a.y, a.x) * (radius * side);
        segs.push(Seg2::Line { a: from, b: leave });
        let start = (leave.y - centre.y).atan2(leave.x - centre.x);
        segs.push(Seg2::Arc { center: centre, radius, start, sweep: side * std::f64::consts::FRAC_PI_2 });
        from = join;
    }
    segs.push(Seg2::Line { a: from, b: middle });
    segs.iter().map(|s| PathSeg::of_sketch(&level, s)).collect()
}

/// What an upright mill fills carried round the closed level `path`: its `section` through its
/// axis (its sketch x across the path, to the left; its sketch y up from the path's height)
/// drawn square to the path where it starts, and swept round it.
pub fn round_the_path(section: Region2, path: &[PathSeg]) -> R<Body> {
    let start = path[0].start();
    let (ahead, _) = path[0].tangents().ok_or_else(|| KernelError::Invalid("a path's direction".into()))?;
    // sketch x to the left of the path, sketch y up: the plane's normal is the path's direction
    let left = Vec3::Z.cross(ahead);
    sweep(&Plane { origin: start, x: left, y: Vec3::Z }, &section, path)
}

pub fn build() -> R<Body> {
    let mut plate = block((0.0, 0.0, -15.0), (120.0, 90.0, 0.0))?;
    for z in [-3.0, -6.0, -9.0] {
        let lap = round_the_path(rect(-3.0, 0.0, 3.0, 20.0), &rounded_loop(&CROSS, 6.0, z))?;
        plate = cut(&plate, &lap)?;
    }
    Ok(plate)
}

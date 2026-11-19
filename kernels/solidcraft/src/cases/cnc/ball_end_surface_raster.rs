//! A plate, the box from (0, 0, −20) to (100, 80, 0), its top finished by a ball-end mill of
//! radius 3, 30 long, upright, in 41 zigzag raster passes: its tip run straight between the
//! points (x, 2j, f(x, 2j)) of f(x, y) = −5 + 4 sin(πx/50) sin(πy/40), x = −5, 0, …, 105, along
//! y = 2j for j = 0 to 40, the even passes along +x and the odd ones back.
//!
//! solidcraft-kernel has no sweep of a solid, so each pass's sweep is built by elementary
//! geometry. The mill is the ball about its centre, 3 above its tip, with the shank above it;
//! a pass runs the centre along a polyline in the upright plane y = 2j, straight from corner to
//! corner, the mill never turning. Below its top, which stays above z = 21 and so above the plate,
//! what the mill passes through is every point within 3 of that polyline (the ball's sweep) and
//! every point straight above one of those (the shank's):
//!
//! - above the polyline, the mill's section square to the pass (from the centre up 27, 6 wide)
//!   moved along it without turning, a slanted slab over each move;
//! - below it, on each move, the half of the cylinder of radius 3 about the move below the
//!   slab's floor, the half disc square to the move carried along it;
//! - at a corner where the pass bends up, the wedge of the ball between the two moves' half
//!   discs: the ball behind the plane square to the move leaving the corner and ahead of the
//!   plane square to the move arriving. Where it bends down that wedge lies above the polyline,
//!   in the slab, and where it runs straight on there is none.
//!
//! The ball's ends past the plate, at x = −5 and 105, the moves between passes there and the
//! plunge and rise at (−5, 0) and (−5, 80) cut nothing. The pieces are joined in one union and
//! cut from the plate.
use crate::solids::*;
use solidcore_record::Seg2;
use solidcraft_kernel::{PathSeg, SweepControl, SweepOrientation, sweep_with};
use std::f64::consts::PI;

fn surface(x: f64, y: f64) -> f64 {
    -5.0 + 4.0 * (PI * x / 50.0).sin() * (PI * y / 40.0).sin()
}

/// The half disc of radius 3 below the sketch x axis.
fn half_disc() -> Region2 {
    let segs = vec![
        Seg2::Arc { center: Vec2::ZERO, radius: 3.0, start: PI, sweep: PI },
        Seg2::Line { a: v2(3.0, 0.0), b: v2(-3.0, 0.0) },
    ];
    Region2 { outer: Loop2 { segs }, holes: vec![] }
}

/// The plane through `c` square to the unit `d` (a direction in the plane y = 0): its sketch y
/// the way up square to `d`, its sketch x along y, its normal `d`.
fn square_to(c: Vec3, d: Vec3) -> Plane {
    let up = Vec3::Y.cross(d);
    let up = if up.z < 0.0 { -up } else { up };
    Plane { origin: c, x: up.cross(d), y: up }
}

/// The part of `body` on the side of the plane through `c` square to `d` that `d` points to
/// (`ahead`), or the other; `None` where the kernel finds nothing there.
fn beside(body: &Body, c: Vec3, d: Vec3, ahead: bool) -> R<Option<Body>> {
    let plane = Plane::from_normal(c, d).ok_or_else(|| KernelError::Invalid("a move's plane".into()))?;
    Ok(solidcraft_kernel::half_space(body, &plane, ahead)?.0)
}

/// The pieces of what the mill sweeps along one pass, its ball's centre through `centres`.
fn pass(centres: &[Vec3]) -> R<Vec<Body>> {
    // the slab: the section through the axis square to y, from the centre up, carried along
    let section = Plane { origin: centres[0], x: Vec3::Y, y: Vec3::Z };
    let path: Vec<PathSeg> = centres.windows(2).map(|m| PathSeg::Line { a: m[0], b: m[1] }).collect();
    let along = SweepControl { orientation: SweepOrientation::Parallel, ..SweepControl::default() };
    let mut pieces = vec![sweep_with(&section, &rect(-3.0, 0.0, 3.0, 27.0), &path, &along)?];
    let dirs: Vec<Vec3> = centres.windows(2).map(|m| m[1] - m[0]).collect();
    for (i, d) in dirs.iter().enumerate() {
        let unit = *d * (1.0 / d.len());
        // the half cylinder below the move
        pieces.push(extrude(&square_to(centres[i], unit), half_disc(), 0.0, d.len())?);
        if i == 0 {
            continue;
        }
        // the wedge at the corner the move starts from, where the pass bends up: the ball
        // between the last move's end and this move's start
        let last = dirs[i - 1] * (1.0 / dirs[i - 1].len());
        // (a straight run's moves can differ by rounding: a bend is one of more than 1e-6, well
        // above that and well below the surface's least, about 4e-3)
        if (last - unit).z < 0.0 && last.dot(unit).clamp(-1.0, 1.0).acos() > 1e-6 {
            let ball = solidcraft_kernel::sphere(centres[i], 3.0)?;
            let missing = || KernelError::Failed("the kernel finds no wedge at a bend".into());
            let behind = beside(&ball, centres[i], unit, false)?.ok_or_else(missing)?;
            pieces.push(beside(&behind, centres[i], last, true)?.ok_or_else(missing)?);
        }
    }
    Ok(pieces)
}

pub fn build() -> R<Body> {
    let plate = block((0.0, 0.0, -20.0), (100.0, 80.0, 0.0))?;
    let mut pieces = Vec::new();
    for j in 0..=40 {
        let y = 2.0 * j as f64;
        let mut xs: Vec<f64> = (0..=22).map(|i| -5.0 + 5.0 * i as f64).collect();
        if j % 2 == 1 {
            xs.reverse();
        }
        let centres: Vec<Vec3> = xs.iter().map(|&x| v3(x, y, surface(x, y) + 3.0)).collect();
        pieces.extend(pass(&centres)?);
    }
    let swept = solidcraft_kernel::union_all(&pieces.iter().collect::<Vec<_>>())?;
    cut(&plate, &swept)
}

//! A T-block 8 high, its bar 20 by 4 and its stem 6 wide, its eight sides drafted by 45° about the
//! neutral plane z = 0, pull +z: the bar's long sides meet in a ridge at 2 and the stem's at 3,
//! below the top, which the draft consumes.
//!
//! SolidCraft's Draft (`draft`) refuses the T, as does solidcore-ops's draft beneath it ("an edge
//! of its offset closes to nothing"), and each refuses the bar or the stem drafted alone, a box
//! whose top the draft consumes. As the case's own set says, the T drafted is the union of its
//! bar and its stem drafted, each the solid under its four sides, each side turned 45° about its
//! bottom edge, above the floor: a convex polyhedron (`convex_polyhedron`) of those planes.
use crate::solids::*;
use solidcraft_kernel::{HalfSpace, convex_polyhedron};

/// The rectangle from (x0, y0) to (x1, y1) on the floor, its four sides drafted by `degrees`
/// about their bottom edges, leaning in as they rise: the half-spaces `n · p <= d` (n the outward
/// unit normal) of the floor and the four sides.
fn drafted(x0: f64, y0: f64, x1: f64, y1: f64, degrees: f64) -> R<Body> {
    let t = degrees.to_radians().tan();
    let side = |n: Vec3, d: f64| {
        let len = n.len();
        HalfSpace { n: n * (1.0 / len), d: d / len }
    };
    convex_polyhedron(&[
        side(v3(0.0, 0.0, -1.0), 0.0),
        // x ≥ x0 + z·t, x ≤ x1 − z·t, and in y the same
        side(v3(-1.0, 0.0, t), -x0),
        side(v3(1.0, 0.0, t), x1),
        side(v3(0.0, -1.0, t), -y0),
        side(v3(0.0, 1.0, t), y1),
    ])
}

pub fn build() -> R<Body> {
    // the bar, and the stem from the bar's front edge, y = 0, out to y = 16
    union(&drafted(0.0, 0.0, 20.0, 4.0, 45.0)?, &drafted(7.0, 0.0, 13.0, 16.0, 45.0)?)
}

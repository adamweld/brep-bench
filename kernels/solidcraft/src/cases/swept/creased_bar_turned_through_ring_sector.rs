//! A ring sector, everything between 8 and 12 from the z axis with |z| ≤ 1 and polar angle from
//! 0° to 60°, grooved by a bar drafted both ways from a parting plane, turned about the z axis
//! from −140° to +140°. With u = (1, 0, 0), v = (0, cos 20°, −sin 20°) and n = (0, sin 20°,
//! cos 20°), the bar is every a u + b v + h n with |h| ≤ 4, 10 + |h| tan 2° ≤ a ≤ 13 − |h| tan 2°
//! and |b| ≤ 2 − |h| tan 2°.
//!
//! solidcore-ops has no sweep of a solid, so the turn is built by elementary geometry, from the
//! kernel's revolves of flat faces about the z axis ([`leading_faces_turned`]). The bar is convex
//! (its sides fold outward at the parting plane): its ends at h = ±4 and its four sides, each two
//! flat pieces meeting at h = 0. It starts clear of the block, so what it takes from it is what its
//! leading faces sweep.
use super::bar_end_turned_into_a_floor::{leading_faces_turned, ring_sector};
use crate::bench::{cut_all, rotated};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let (s20, c20) = 20f64.to_radians().sin_cos();
    let draft = 2f64.to_radians().tan();
    let (u, v, n) = ([1.0, 0.0, 0.0], [0.0, c20, -s20], [0.0, s20, c20]);
    let point = |a: f64, b: f64, h: f64| -> [f64; 3] {
        let mut p = [0.0; 3];
        for i in 0..3 {
            p[i] = a * u[i] + b * v[i] + h * n[i];
        }
        p
    };
    // the rectangle 10 ≤ a ≤ 13, |b| ≤ 2 shrunk by |h| tan 2° on every side, at h = −4, 0 and 4
    let mut corners = Vec::new();
    for h in [-4.0f64, 0.0, 4.0] {
        let k = h.abs() * draft;
        for (a, b) in [(10.0 + k, -2.0 + k), (13.0 - k, -2.0 + k), (13.0 - k, 2.0 - k), (10.0 + k, 2.0 - k)] {
            corners.push(point(a, b, h));
        }
    }
    let mut faces = vec![vec![0, 1, 2, 3], vec![8, 9, 10, 11]];
    for i in 0..4 {
        let j = (i + 1) % 4;
        faces.push(vec![i, j, j + 4, i + 4]);
        faces.push(vec![i + 4, j + 4, j + 8, i + 8]);
    }
    let swept = leading_faces_turned(&corners, &faces, 280.0)?
        .into_iter()
        .map(|b| rotated(b, [0.0; 3], [0.0, 0.0, 1.0], -140.0))
        .collect::<Result<Vec<_>, _>>()?;
    cut_all(ring_sector()?, swept)
}

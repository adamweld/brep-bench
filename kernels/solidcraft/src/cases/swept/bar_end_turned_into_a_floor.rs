//! A ring sector, everything between 8 and 12 from the z axis with |z| ≤ 1 and polar angle from
//! 0° to 60°, cut by a leaning bar turned about the z axis from −140° to +140°. The bar's end is
//! the rectangle {0 ≤ a cos 5° + b sin 5° ≤ 3, |b cos 5° − a sin 5°| ≤ 2} in the plane of the
//! points (10, 0, 0) + a u + b v, u = (cos 20°, 0, −sin 20°), v = (0, 1, 0); the bar is that
//! rectangle carried 3 along n = (sin 20°, 0, cos 20°).
//!
//! solidcore-ops has no sweep of a solid, so the turn is built by elementary geometry, from the
//! kernel's revolves of flat faces about the z axis ([`leading_faces_turned`]). The bar starts
//! clear of the block, so what it takes from it is what its leading faces sweep.
use crate::bench::{cut_all, polygon, rotated};
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::Body;

type P = [f64; 3];

fn sub(a: P, b: P) -> P {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: P, b: P) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: P, b: P) -> P {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// The solids the faces of a convex polyhedron sweep where they lead, as it turns `degrees`
/// about the z axis, right-handed, from where it is drawn: its `corners`, and its `faces` as
/// lists of corners in order round each.
///
/// A point the polyhedron passes over that is outside it where it starts is entered across its
/// boundary where the boundary moves towards it: where a face's outward normal n leads the
/// velocity ẑ × p. That is where m · p > 0, m = n × ẑ: the half of the face's plane on one side of
/// the plane through the z axis square to m, whose line across the face is its contact. So the
/// polyhedron's sweep is where it starts and what the leading part of each face sweeps (a face
/// square to the axis turns in its own plane, and leads nowhere). A leading part meets each
/// circle about the axis at most once, so it turns without folding over itself: the kernel's
/// revolve of that flat region.
pub fn leading_faces_turned(corners: &[P], faces: &[Vec<usize>], degrees: f64) -> Result<Vec<Body>, Refusal> {
    let n = corners.len() as f64;
    let centre = corners.iter().fold([0.0; 3], |c, p| [c[0] + p[0] / n, c[1] + p[1] / n, c[2] + p[2] / n]);
    let mut swept = Vec::new();
    for face in faces {
        let ring: Vec<P> = face.iter().map(|&i| corners[i]).collect();
        let mut normal = cross(sub(ring[1], ring[0]), sub(ring[2], ring[0]));
        if dot(normal, sub(ring[0], centre)) < 0.0 {
            normal = [-normal[0], -normal[1], -normal[2]];
        }
        let m = cross(normal, [0.0, 0.0, 1.0]);
        if dot(m, m).sqrt() < 1e-9 * dot(normal, normal).sqrt() {
            continue;
        }
        // the face clipped to m · p ≥ 0, where the contact line crosses its sides
        let mut lead = Vec::new();
        for (k, &p) in ring.iter().enumerate() {
            let q = ring[(k + 1) % ring.len()];
            let (sp, sq) = (dot(m, p), dot(m, q));
            if sp >= 0.0 {
                lead.push(p);
            }
            if (sp > 0.0 && sq < 0.0) || (sp < 0.0 && sq > 0.0) {
                let t = sp / (sp - sq);
                lead.push([p[0] + t * (q[0] - p[0]), p[1] + t * (q[1] - p[1]), p[2] + t * (q[2] - p[2])]);
            }
        }
        lead.dedup_by(|a, b| dot(sub(*a, *b), sub(*a, *b)) < 1e-24);
        if lead.len() < 3 {
            continue;
        }
        let area = (1..lead.len() - 1)
            .map(|k| dot(cross(sub(lead[k], lead[0]), sub(lead[k + 1], lead[0])), normal))
            .sum::<f64>();
        if area.abs() < 1e-9 * dot(normal, normal).sqrt() {
            continue;
        }
        swept.push(revolve(&polygon(&lead), [0.0; 3], [0.0, 0.0, 1.0], degrees.to_radians(), true)?);
    }
    Ok(swept)
}

/// The block the turned tools cut: the rectangle 8 ≤ x ≤ 12, |z| ≤ 1 at y = 0 revolved from 0°
/// to 60°.
pub fn ring_sector() -> Result<Body, Refusal> {
    let section = polygon(&[[8.0, 0.0, -1.0], [12.0, 0.0, -1.0], [12.0, 0.0, 1.0], [8.0, 0.0, 1.0]]);
    Ok(revolve(&section, [0.0; 3], [0.0, 0.0, 1.0], 60f64.to_radians(), true)?)
}

pub fn build() -> Result<Body, Refusal> {
    let (s20, c20) = 20f64.to_radians().sin_cos();
    let (s5, c5) = 5f64.to_radians().sin_cos();
    let (u, v, n) = ([c20, 0.0, -s20], [0.0, 1.0, 0.0], [s20, 0.0, c20]);
    let point = |a: f64, b: f64, s: f64| -> P {
        let mut p = [10.0, 0.0, 0.0];
        for i in 0..3 {
            p[i] += a * u[i] + b * v[i] + s * n[i];
        }
        p
    };
    // the end's corners, (a, b) = p (cos 5°, sin 5°) + q (−sin 5°, cos 5°), at s = 0 and 3
    let mut corners = Vec::new();
    for s in [0.0, 3.0] {
        for (p, q) in [(0.0, -2.0), (3.0, -2.0), (3.0, 2.0), (0.0, 2.0)] {
            corners.push(point(p * c5 - q * s5, p * s5 + q * c5, s));
        }
    }
    let mut faces = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7]];
    for i in 0..4 {
        let j = (i + 1) % 4;
        faces.push(vec![i, j, j + 4, i + 4]);
    }
    let start = -140.0;
    let swept = leading_faces_turned(&corners, &faces, 280.0)?
        .into_iter()
        .map(|b| rotated(b, [0.0; 3], [0.0, 0.0, 1.0], start))
        .collect::<Result<Vec<_>, _>>()?;
    cut_all(ring_sector()?, swept)
}

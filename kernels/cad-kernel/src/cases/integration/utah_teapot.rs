//! Martin Newell's teapot as a solid: his 32 bicubic Bézier patches in his own coordinates, z up,
//! the union of four solids each bounded by some of them and closed by flat faces — the body
//! (rim, body and bottom; the disc of radius 1.4 on z = 2.4 inside the rim), the lid (knob and
//! lid; the disc of radius 1.3 on z = 2.4), the spout (its base on x = 1.7, its mouth on z = 2.4
//! inside the lip) and the handle (each end's loop, a plane). The body, the lid and the bottom
//! are rings of four patches a quarter turn apart, each from a profile of four points (r, z): row
//! i is (r, 0, z), (r, −0.56r, z), (0.56r, −r, z), (0, −r, z) (0.5625 in the knob's second row).
//! The spout and the handle are halves mirrored in y = 0.
//!
//! Built (cad-kernel 0.6.2), its triangles closed and enclosing 25.980, but its own measures are
//! short where a sheet closes to a point: the body alone measures 23.92584, short of its exact
//! volume 24.40134 by 0.47550, which is exactly what the bottom's four patches add to it, and the
//! body's and the lid's areas are short by 5.217 and 0.685. The spout's and the handle's measures
//! agree with exact integration to 1e-11.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{face, sheet, Profile, ProfileEdge};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::nurbs::{BSpline, Net};
use cad_kernel::sew::sewn;
use cad_kernel::topo::Brep;
use std::sync::Arc;

type Row = [V; 4];
type Patch = [Row; 4];

/// Profiles (r, z) of the rings.
const RIM: [[f64; 2]; 4] = [[1.4, 2.4], [1.3375, 2.53125], [1.4375, 2.53125], [1.5, 2.4]];
const UPPER_BODY: [[f64; 2]; 4] = [[1.5, 2.4], [1.75, 1.875], [2.0, 1.35], [2.0, 0.9]];
const LOWER_BODY: [[f64; 2]; 4] = [[2.0, 0.9], [2.0, 0.45], [1.5, 0.225], [1.5, 0.15]];
const BOTTOM: [[f64; 2]; 4] = [[1.5, 0.15], [1.5, 0.075], [1.425, 0.0], [0.0, 0.0]];
const KNOB: [[f64; 2]; 4] = [[0.0, 3.15], [0.8, 3.15], [0.0, 2.85], [0.2, 2.7]];
const LID: [[f64; 2]; 4] = [[0.2, 2.7], [0.4, 2.55], [1.3, 2.55], [1.3, 2.4]];

/// The halves of the handle and the spout on the side y ≤ 0.
const HANDLE: [Patch; 2] = [
    [[[-1.6, 0.0, 2.025], [-1.6, -0.3, 2.025], [-1.5, -0.3, 2.25], [-1.5, 0.0, 2.25]],
     [[-2.3, 0.0, 2.025], [-2.3, -0.3, 2.025], [-2.5, -0.3, 2.25], [-2.5, 0.0, 2.25]],
     [[-2.7, 0.0, 2.025], [-2.7, -0.3, 2.025], [-3.0, -0.3, 2.25], [-3.0, 0.0, 2.25]],
     [[-2.7, 0.0, 1.8], [-2.7, -0.3, 1.8], [-3.0, -0.3, 1.8], [-3.0, 0.0, 1.8]]],
    [[[-2.7, 0.0, 1.8], [-2.7, -0.3, 1.8], [-3.0, -0.3, 1.8], [-3.0, 0.0, 1.8]],
     [[-2.7, 0.0, 1.575], [-2.7, -0.3, 1.575], [-3.0, -0.3, 1.35], [-3.0, 0.0, 1.35]],
     [[-2.5, 0.0, 1.125], [-2.5, -0.3, 1.125], [-2.65, -0.3, 0.9375], [-2.65, 0.0, 0.9375]],
     [[-2.0, 0.0, 0.9], [-2.0, -0.3, 0.9], [-1.9, -0.3, 0.6], [-1.9, 0.0, 0.6]]],
];
const SPOUT: [Patch; 2] = [
    [[[1.7, 0.0, 1.425], [1.7, -0.66, 1.425], [1.7, -0.66, 0.6], [1.7, 0.0, 0.6]],
     [[2.6, 0.0, 1.425], [2.6, -0.66, 1.425], [3.1, -0.66, 0.825], [3.1, 0.0, 0.825]],
     [[2.3, 0.0, 2.1], [2.3, -0.25, 2.1], [2.4, -0.25, 2.025], [2.4, 0.0, 2.025]],
     [[2.7, 0.0, 2.4], [2.7, -0.25, 2.4], [3.3, -0.25, 2.4], [3.3, 0.0, 2.4]]],
    [[[2.7, 0.0, 2.4], [2.7, -0.25, 2.4], [3.3, -0.25, 2.4], [3.3, 0.0, 2.4]],
     [[2.8, 0.0, 2.475], [2.8, -0.25, 2.475], [3.525, -0.25, 2.49375], [3.525, 0.0, 2.49375]],
     [[2.9, 0.0, 2.475], [2.9, -0.15, 2.475], [3.45, -0.15, 2.5125], [3.45, 0.0, 2.5125]],
     [[2.8, 0.0, 2.4], [2.8, -0.15, 2.4], [3.2, -0.15, 2.4], [3.2, 0.0, 2.4]]],
];

/// `p` turned `q` quarter turns round the z axis, from +x toward −y.
fn quarter_turn([x, y, z]: V, q: usize) -> V {
    (0..q).fold([x, y, z], |[x, y, z], _| [y, -x, z])
}

/// The four patches of a ring, row i from the profile's point i, each a quarter turn on.
fn ring(profile: &[[f64; 2]; 4], knob: bool) -> Vec<Patch> {
    let first: Patch = std::array::from_fn(|i| {
        let [r, z] = profile[i];
        let k = if knob && i == 1 { 0.5625 } else { 0.56 };
        [[r, 0., z], [r, -k * r, z], [k * r, -r, z], [0., -r, z]]
    });
    (0..4).map(|q| first.map(|row| row.map(|p| quarter_turn(p, q)))).collect()
}

/// A half mirrored in the plane y = 0, each row reversed so it runs on from the half.
fn mirrored(patch: &Patch) -> Patch {
    patch.map(|row| std::array::from_fn(|j| { let [x, y, z] = row[3 - j]; [x, -y, z] }))
}

fn bezier(row: &Row) -> Result<BSpline, String> {
    BSpline::new(3, vec![0., 0., 0., 0., 1., 1., 1., 1.], row.to_vec())
}

fn patch(p: &Patch) -> Result<Brep, String> {
    let knots = vec![0., 0., 0., 0., 1., 1., 1., 1.];
    sheet(Net { du: 3, dv: 3, uknots: knots.clone(), vknots: knots,
                poles: p.iter().map(|row| row.to_vec()).collect(), weights: None })
}

/// The flat face inside the closed loop of cubic Bézier curves on `rows`, in the plane with
/// normal `normal` (sewing turns it to face out).
fn cap(rows: &[Row], normal: V) -> Result<Brep, String> {
    let loop_ = rows.iter().map(|r| Ok(ProfileEdge::Spline(Arc::new(bezier(r)?))))
        .collect::<Result<Vec<_>, String>>()?;
    face(&Profile { origin: rows[0][0], normal, loops: vec![loop_], names: Vec::new() })
}

/// The solid the patches and the caps bound.
fn solid(patches: &[Patch], caps: Vec<Brep>) -> Result<Brep, String> {
    let mut faces = patches.iter().map(patch).collect::<Result<Vec<_>, String>>()?;
    faces.extend(caps);
    Ok(sewn(&faces, tolerance(4.))?)
}

/// The normal of the plane through a row's ends and its second control point.
fn normal(row: &Row) -> V {
    let [a, b, c] = [row[0], row[1], row[3]];
    let (u, v) = ([b[0] - a[0], b[1] - a[1], b[2] - a[2]], [c[0] - a[0], c[1] - a[1], c[2] - a[2]]);
    [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]]
}

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rim = ring(&RIM, false);
    let lid_ring = ring(&LID, false);
    let body_patches = [rim.clone(), ring(&UPPER_BODY, false), ring(&LOWER_BODY, false),
                        ring(&BOTTOM, false)].concat();
    let body = solid(&body_patches, vec![cap(&rim.iter().map(|p| p[0]).collect::<Vec<_>>(), up)?])?;
    let lid_patches = [ring(&KNOB, true), lid_ring.clone()].concat();
    let lid = solid(&lid_patches, vec![cap(&lid_ring.iter().map(|p| p[3]).collect::<Vec<_>>(), up)?])?;
    let halves = |h: &[Patch; 2]| vec![h[0], h[1], mirrored(&h[0]), mirrored(&h[1])];
    let ends = |h: &[Patch; 2]| -> Result<Vec<Brep>, String> {
        let (first, last) = ([h[0][0], mirrored(&h[0])[0]], [h[1][3], mirrored(&h[1])[3]]);
        Ok(vec![cap(&first, normal(&h[0][0]))?, cap(&last, normal(&h[1][3]))?])
    };
    let spout = solid(&halves(&SPOUT), ends(&SPOUT)?)?;
    let handle = solid(&halves(&HANDLE), ends(&HANDLE)?)?;
    let mut teapot = combined(&body, &lid, Op::Union, 0.)?;
    for part in [&spout, &handle] {
        teapot = combined(&teapot, part, Op::Union, 0.)?;
    }
    Ok(teapot)
}

//! The Utah teapot: Martin Newell's 32 bicubic Bézier patches (with Frank Crow's four of the
//! bottom), in his own coordinates, made a solid as the union of four solids, each bounded by
//! some of the patches and closed by flat faces:
//!
//! - the body: the rim, the upper and lower body and the bottom, closed by the disk inside the
//!   rim's inner edge in the plane z = 2.4;
//! - the lid: the knob and the lid, closed by the disk inside its edge in the plane z = 2.4;
//! - the spout, closed at its base in the plane x = 1.7 and at its mouth in the plane z = 2.4;
//! - the handle, closed at each end in that end's plane.
//!
//! Each patch is a sheet of its own (`tspline_sheet` of the tensor Bézier surface), each flat
//! face a planar patch of the region inside its loop of cubic Bézier curves, the rows of the
//! patches it closes (`patch_region`), and each solid's faces are stitched along the edges they
//! share. The rings are a profile's patch turned three quarter turns about the z axis; the spout
//! and the handle are their halves on y ≤ 0 and those mirrored in the plane y = 0.
use crate::solids::*;
use solidcore_record::Seg2;
use solidcraft_geom::bspline::NurbsSurface;
use solidcraft_geom::tspline::TSpline;

type Net = [[[f64; 3]; 4]; 4];

/// The profiles (r, z) of the rings.
const RIM: [(f64, f64); 4] = [(1.4, 2.4), (1.3375, 2.53125), (1.4375, 2.53125), (1.5, 2.4)];
const UPPER_BODY: [(f64, f64); 4] = [(1.5, 2.4), (1.75, 1.875), (2.0, 1.35), (2.0, 0.9)];
const LOWER_BODY: [(f64, f64); 4] = [(2.0, 0.9), (2.0, 0.45), (1.5, 0.225), (1.5, 0.15)];
const BOTTOM: [(f64, f64); 4] = [(1.5, 0.15), (1.5, 0.075), (1.425, 0.0), (0.0, 0.0)];
const KNOB: [(f64, f64); 4] = [(0.0, 3.15), (0.8, 3.15), (0.0, 2.85), (0.2, 2.7)];
const LID: [(f64, f64); 4] = [(0.2, 2.7), (0.4, 2.55), (1.3, 2.55), (1.3, 2.4)];

/// The spout's and the handle's patches on y ≤ 0, each row by row.
const SPOUT: [Net; 2] = [
    [
        [[1.7, 0.0, 1.425], [1.7, -0.66, 1.425], [1.7, -0.66, 0.6], [1.7, 0.0, 0.6]],
        [[2.6, 0.0, 1.425], [2.6, -0.66, 1.425], [3.1, -0.66, 0.825], [3.1, 0.0, 0.825]],
        [[2.3, 0.0, 2.1], [2.3, -0.25, 2.1], [2.4, -0.25, 2.025], [2.4, 0.0, 2.025]],
        [[2.7, 0.0, 2.4], [2.7, -0.25, 2.4], [3.3, -0.25, 2.4], [3.3, 0.0, 2.4]],
    ],
    [
        [[2.7, 0.0, 2.4], [2.7, -0.25, 2.4], [3.3, -0.25, 2.4], [3.3, 0.0, 2.4]],
        [[2.8, 0.0, 2.475], [2.8, -0.25, 2.475], [3.525, -0.25, 2.49375], [3.525, 0.0, 2.49375]],
        [[2.9, 0.0, 2.475], [2.9, -0.15, 2.475], [3.45, -0.15, 2.5125], [3.45, 0.0, 2.5125]],
        [[2.8, 0.0, 2.4], [2.8, -0.15, 2.4], [3.2, -0.15, 2.4], [3.2, 0.0, 2.4]],
    ],
];
const HANDLE: [Net; 2] = [
    [
        [[-1.6, 0.0, 2.025], [-1.6, -0.3, 2.025], [-1.5, -0.3, 2.25], [-1.5, 0.0, 2.25]],
        [[-2.3, 0.0, 2.025], [-2.3, -0.3, 2.025], [-2.5, -0.3, 2.25], [-2.5, 0.0, 2.25]],
        [[-2.7, 0.0, 2.025], [-2.7, -0.3, 2.025], [-3.0, -0.3, 2.25], [-3.0, 0.0, 2.25]],
        [[-2.7, 0.0, 1.8], [-2.7, -0.3, 1.8], [-3.0, -0.3, 1.8], [-3.0, 0.0, 1.8]],
    ],
    [
        [[-2.7, 0.0, 1.8], [-2.7, -0.3, 1.8], [-3.0, -0.3, 1.8], [-3.0, 0.0, 1.8]],
        [[-2.7, 0.0, 1.575], [-2.7, -0.3, 1.575], [-3.0, -0.3, 1.35], [-3.0, 0.0, 1.35]],
        [[-2.5, 0.0, 1.125], [-2.5, -0.3, 1.125], [-2.65, -0.3, 0.9375], [-2.65, 0.0, 0.9375]],
        [[-2.0, 0.0, 0.9], [-2.0, -0.3, 0.9], [-1.9, -0.3, 0.6], [-1.9, 0.0, 0.6]],
    ],
];

/// The four patches of a ring: row i of the first, spanning the quarter from +x to −y, from the
/// profile's point i (with the knob's row 1 as the data set has it), the others that turned a
/// quarter turn at a time from +x toward −y.
fn ring(profile: [(f64, f64); 4], knob: bool) -> Vec<Net> {
    let mut first = [[[0.0; 3]; 4]; 4];
    for (i, &(r, z)) in profile.iter().enumerate() {
        let k = if knob && i == 1 { 0.45 } else { 0.56 * r };
        first[i] = [[r, 0.0, z], [r, -k, z], [k, -r, z], [0.0, -r, z]];
    }
    let mut out = vec![first];
    for q in 1..4 {
        out.push(out[q - 1].map(|row| row.map(|[x, y, z]| [y, -x, z])));
    }
    out
}

/// A half's patch mirrored in the plane y = 0, each row reversed so that it runs on from the
/// half's.
fn mirrored(net: &Net) -> Net {
    net.map(|row| {
        let mut r = row.map(|[x, y, z]| [x, -y, z]);
        r.reverse();
        r
    })
}

/// The patch as a sheet: SolidCraft's NURBS sheet is a T-spline's, and a T-mesh without
/// T-junctions is the tensor surface itself (as in `nurbs/bezier_dome`).
fn sheet(net: &Net) -> Result<Body, crate::Refusal> {
    let knots = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    // u along the rows, v along the columns, u fastest
    let pts: Vec<Vec3> = (0..4).flat_map(|j| (0..4).map(move |i| net[i][j])).map(|[x, y, z]| v3(x, y, z)).collect();
    let s = NurbsSurface::new([3, 3], [knots.clone(), knots], &pts, &[1.0; 16]).map_err(|e| e.to_string())?;
    let ts = TSpline::from_nurbs(&s).map_err(|e| e.to_string())?;
    Ok(solidcraft_kernel::tspline_sheet(&ts)?)
}

/// The flat face on `plane` inside the closed loop of the cubic Bézier curves over `rows`, each
/// row's control points, end to end.
fn cap(plane: Plane, rows: &[[[f64; 3]; 4]]) -> R<Body> {
    let at = |[x, y, z]: [f64; 3]| {
        let d = v3(x, y, z) - plane.origin;
        v2(d.dot(plane.x), d.dot(plane.y))
    };
    let segs = rows.iter().map(|r| Seg2::Cubic { p0: at(r[0]), p1: at(r[1]), p2: at(r[2]), p3: at(r[3]) }).collect();
    solidcraft_kernel::patch_region(&plane, &Region2 { outer: Loop2 { segs }, holes: vec![] })
}

/// The solid bounded by the patches `nets` and the flat faces `caps`, stitched.
fn solid(nets: &[Net], caps: Vec<Body>) -> Result<Body, crate::Refusal> {
    let mut faces = nets.iter().map(sheet).collect::<Result<Vec<_>, _>>()?;
    faces.extend(caps);
    Ok(solidcraft_kernel::stitch(&faces.iter().collect::<Vec<_>>(), 1e-6)?)
}

/// The plane through `a` and `b` (in the plane y = 0) parallel to the y axis.
fn upright_through(a: [f64; 3], b: [f64; 3]) -> Plane {
    let d = v3(b[0] - a[0], 0.0, b[2] - a[2]);
    Plane { origin: v3(a[0], 0.0, a[2]), x: d * (1.0 / d.len()), y: Vec3::Y }
}

/// A half's patches and their mirror images.
fn halves(nets: &[Net; 2]) -> Vec<Net> {
    vec![nets[0], nets[1], mirrored(&nets[0]), mirrored(&nets[1])]
}

/// The loop of row `row` of a half's patch and the same row of its mirror image.
fn end(net: &Net, row: usize) -> [[[f64; 3]; 4]; 2] {
    [net[row], mirrored(net)[row]]
}

pub fn build() -> Result<Body, crate::Refusal> {
    let level = Plane { origin: v3(0.0, 0.0, 2.4), x: Vec3::X, y: Vec3::Y };
    let row = |nets: &[Net], i: usize| nets.iter().map(|n| n[i]).collect::<Vec<_>>();
    // the body
    let rim = ring(RIM, false);
    let mut nets = rim.clone();
    for profile in [UPPER_BODY, LOWER_BODY, BOTTOM] {
        nets.extend(ring(profile, false));
    }
    let body = solid(&nets, vec![cap(level, &row(&rim, 0))?])?;
    // the lid
    let lid = ring(LID, false);
    let mut nets = ring(KNOB, true);
    nets.extend(lid.iter().copied());
    let lid = solid(&nets, vec![cap(level, &row(&lid, 3))?])?;
    // the spout, closed at its base and its mouth
    let base = Plane { origin: v3(1.7, 0.0, 0.0), x: Vec3::Y, y: Vec3::Z };
    let spout = solid(&halves(&SPOUT), vec![cap(base, &end(&SPOUT[0], 0))?, cap(level, &end(&SPOUT[1], 3))?])?;
    // the handle, closed at each end
    let upper = upright_through(HANDLE[0][0][0], HANDLE[0][0][3]);
    let lower = upright_through(HANDLE[1][3][0], HANDLE[1][3][3]);
    let handle = solid(&halves(&HANDLE), vec![cap(upper, &end(&HANDLE[0], 0))?, cap(lower, &end(&HANDLE[1], 3))?])?;
    let teapot = union(&body, &lid)?;
    let teapot = union(&teapot, &spout)?;
    Ok(union(&teapot, &handle)?)
}

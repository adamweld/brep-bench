//! A 30-tooth bevel gear's blank with one tooth space cut by a straight-sided tooth of its
//! 15-tooth pinion, the shafts square and meeting at O: the pinion turns by t about the x axis
//! while the gear turns by −t/2 about the z axis, t from −60° to 60°.
//!
//! Both turns hold O still and the tooth is a frustum of a pyramid with apex O, so inside the
//! blank (which the tooth's end planes never reach) what it sweeps is a cone with apex O: the
//! rays through O over one plane region. Here that region is taken in the plane square to g
//! through the outer pitch point P, where every plane through O is a line. There each place of the
//! tooth is a convex quadrilateral, the image of its trapezoid, and the region is the union of
//! those quadrilaterals as t runs. Its boundary, on the blank's side of the line u = −3 (the
//! blank stays at u ≥ −2 there), is a chain of curves each known at every t in closed form: the
//! track of a corner of the quadrilateral, or the envelope of one of its sides (the point of the
//! side that moves along the side's own line). In order, from the line u = −3 round to it again:
//! the track of the outer flank's tip-side corner, the envelope of that flank (taking over where
//! its moving point reaches the corner), the track of the flank's root-side corner from where the
//! two cross, the envelope of the root side (taking over where its moving point leaves that
//! corner), and the same mirrored in w (the motion run backwards is the mirror image).
//!
//! Each curve is interpolated through its exact points by the kernel's cubic fit, and the closed
//! chain scaled about O to the planes at 20 and 38 from O (both clear of the blank, which lies
//! between 23.5 and 33.6): the kernel's loft between the two is the cone, ruled through O. It is
//! cut from the blank, its meridian quadrilateral revolved.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, interpolate_curve, loft_profile_brep, make_line, revolve_profile_brep,
    BooleanOperation, BrepSolid, NurbsCurve, Vec3,
};
use std::f64::consts::{FRAC_PI_2, TAU};

const S5: f64 = 2.236_067_977_499_79;

/// The plane's frame: g (out from O, through P), d (from the pinion into the gear) and e = y.
fn frame() -> (Vec3, Vec3, Vec3) {
    (p(2.0 / S5, 0.0, 1.0 / S5), p(-1.0 / S5, 0.0, 2.0 / S5), p(0.0, 1.0, 0.0))
}

/// The pitch plane's distance from O.
fn s0() -> f64 {
    15.0 * S5
}

/// The tooth's trapezoid in the plane, (u, w) along d and e from P, its corners in order round.
fn corners() -> [[f64; 2]; 4] {
    let t20 = 20f64.to_radians().tan();
    [
        [-2.5, FRAC_PI_2 + 2.5 * t20],
        [2.5, FRAC_PI_2 - 2.5 * t20],
        [2.5, -(FRAC_PI_2 - 2.5 * t20)],
        [-2.5, -(FRAC_PI_2 + 2.5 * t20)],
    ]
}

/// The tooth's motion seen from the blank at t: by t about the x axis, then t/2 about the z axis.
fn moved(t: f64, x: Vec3) -> Vec3 {
    let (c, s) = (t.cos(), t.sin());
    let y = p(x.x, c * x.y - s * x.z, s * x.y + c * x.z);
    let (c, s) = ((t / 2.0).cos(), (t / 2.0).sin());
    p(c * y.x - s * y.y, s * y.x + c * y.y, y.z)
}

/// Where corner `k` of the tooth stands at t, in the plane.
fn corner(k: usize, t: f64) -> [f64; 2] {
    let (g, d, e) = frame();
    let q = corners()[k % 4];
    let x = moved(t, g.scale(s0()).add(d.scale(q[0])).add(e.scale(q[1])));
    let k = s0() / x.dot(g);
    [k * x.dot(d), k * x.dot(e)]
}

/// How fast corner `k` moves in the plane at t (exactly: the motion's and the projection's rates).
fn rate(k: usize, t: f64) -> [f64; 2] {
    let (g, d, e) = frame();
    let q = corners()[k % 4];
    let x0 = g.scale(s0()).add(d.scale(q[0])).add(e.scale(q[1]));
    // y = Rx(t) x0 and its rate; then x = Rz(t/2) y and its rate.
    let (c, s) = (t.cos(), t.sin());
    let y = p(x0.x, c * x0.y - s * x0.z, s * x0.y + c * x0.z);
    let dy = p(0.0, -s * x0.y - c * x0.z, c * x0.y - s * x0.z);
    let (c, s) = ((t / 2.0).cos(), (t / 2.0).sin());
    let x = p(c * y.x - s * y.y, s * y.x + c * y.y, y.z);
    let dx = p(
        c * dy.x - s * dy.y - 0.5 * (s * y.x + c * y.y),
        s * dy.x + c * dy.y + 0.5 * (c * y.x - s * y.y),
        dy.z,
    );
    let (n, dn) = (x.dot(g), dx.dot(g));
    let r = |a: Vec3| s0() * (dx.dot(a) * n - x.dot(a) * dn) / (n * n);
    [r(d), r(e)]
}

fn cross(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}

/// Where along side `k` (from corner k to k + 1) its envelope point lies at t: the point of the
/// side that moves along the side's own line.
fn along(k: usize, t: f64) -> f64 {
    let (a, b) = (corner(k, t), corner(k + 1, t));
    let (da, db) = (rate(k, t), rate(k + 1, t));
    let side = [b[0] - a[0], b[1] - a[1]];
    -cross(side, da) / cross(side, [db[0] - da[0], db[1] - da[1]])
}

fn envelope(k: usize, t: f64) -> [f64; 2] {
    let (a, b, l) = (corner(k, t), corner(k + 1, t), along(k, t));
    [a[0] + l * (b[0] - a[0]), a[1] + l * (b[1] - a[1])]
}

/// The root of `f` between `a` and `b`, by bisection.
fn root(f: impl Fn(f64) -> f64, mut a: f64, mut b: f64) -> Result<f64, Refusal> {
    let (mut fa, fb) = (f(a), f(b));
    if fa * fb > 0.0 {
        return Err(Refusal::from("a hand-off of the tooth space's boundary is not bracketed"));
    }
    for _ in 0..200 {
        let m = 0.5 * (a + b);
        let fm = f(m);
        if (fm < 0.0) == (fa < 0.0) {
            a = m;
            fa = fm;
        } else {
            b = m;
        }
    }
    Ok(0.5 * (a + b))
}

/// Where the flank's envelope crosses the root-side corner's track: (envelope t, corner t), by
/// Newton's method from a start read off the curves.
fn crossing(start: [f64; 2]) -> Result<[f64; 2], Refusal> {
    let gap = |v: [f64; 2]| {
        let (a, b) = (envelope(0, v[0]), corner(1, v[1]));
        [a[0] - b[0], a[1] - b[1]]
    };
    let mut v = start;
    for _ in 0..60 {
        let r = gap(v);
        if r[0].hypot(r[1]) < 1e-12 {
            return Ok(v);
        }
        let h = 1e-8;
        let (a, b) = (gap([v[0] + h, v[1]]), gap([v[0], v[1] + h]));
        let j = [[(a[0] - r[0]) / h, (b[0] - r[0]) / h], [(a[1] - r[1]) / h, (b[1] - r[1]) / h]];
        let det = j[0][0] * j[1][1] - j[0][1] * j[1][0];
        v = [v[0] - (j[1][1] * r[0] - j[0][1] * r[1]) / det, v[1] - (-j[1][0] * r[0] + j[0][0] * r[1]) / det];
    }
    Err(Refusal::from("the flank's envelope and the root corner's track do not cross"))
}

/// A point of the plane, (u, w), in space at distance `at` from O along the rays through O.
fn place(q: [f64; 2], at: f64) -> Vec3 {
    let (g, d, e) = frame();
    g.scale(s0()).add(d.scale(q[0])).add(e.scale(q[1])).scale(at / s0())
}

/// The cubic through `points` (in the plane), at distance s0 from O.
fn fit(points: &[[f64; 2]]) -> Result<NurbsCurve, Refusal> {
    let xs: Vec<Vec3> = points.iter().map(|&q| place(q, s0())).collect();
    let mut s = vec![0.0];
    for w in xs.windows(2) {
        s.push(s.last().unwrap() + w[1].sub(w[0]).length());
    }
    let total = *s.last().unwrap();
    let parameters: Vec<f64> = s.iter().map(|x| x / total).collect();
    Ok(interpolate_curve(&xs, 3, &parameters)?)
}

/// `n` + 1 points of `curve` for t from `a` to `b`, its ends replaced by `ends` where given.
fn track(curve: impl Fn(f64) -> [f64; 2], a: f64, b: f64, n: usize, ends: [Option<[f64; 2]>; 2]) -> Vec<[f64; 2]> {
    let mut points: Vec<[f64; 2]> = (0..=n).map(|i| curve(a + (b - a) * i as f64 / n as f64)).collect();
    if let Some(q) = ends[0] {
        points[0] = q;
    }
    if let Some(q) = ends[1] {
        points[n] = q;
    }
    points
}

/// `curve` scaled about O by `k`.
fn scaled(curve: &NurbsCurve, k: f64) -> NurbsCurve {
    let mut out = curve.clone();
    for c in &mut out.control_points {
        c.x *= k;
        c.y *= k;
        c.z *= k;
    }
    out
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let deg = |d: f64| d.to_radians();
    // The hand-offs along the upper half (w > 0), in t.
    let clip = root(|t| corner(0, t)[0] + 3.0, deg(20.0), deg(45.0))?;
    let onto_flank = root(|t| along(0, t), deg(15.0), deg(35.0))?;
    let [off_flank, onto_corner] = crossing([deg(2.0), deg(-25.0)])?;
    let onto_root = root(|t| along(1, t), deg(0.0), deg(10.0))?;
    let x = envelope(0, off_flank);
    let n = 96;
    let mirror = |q: &[f64; 2]| [q[0], -q[1]];
    // Upper half, from the line u = −3 to the middle of the root side (w = 0).
    let start = corner(0, clip);
    let a = corner(0, onto_flank);
    let b = corner(1, onto_root);
    let upper = vec![
        track(|t| corner(0, t), clip, onto_flank, n, [Some([-3.0, start[1]]), Some(a)]),
        track(|t| envelope(0, t), onto_flank, off_flank, n, [Some(a), Some(x)]),
        track(|t| corner(1, t), onto_corner, onto_root, n, [Some(x), Some(b)]),
        track(|t| envelope(1, t), onto_root, 0.0, n, [Some(b), Some([envelope(1, 0.0)[0], 0.0])]),
    ];
    // The whole loop: the upper half, then its mirror image run back, then the line u = −3.
    let mut loop_points: Vec<Vec<[f64; 2]>> = upper.clone();
    for piece in upper.iter().rev() {
        loop_points.push(piece.iter().rev().map(mirror).collect());
    }
    let mut curves = Vec::new();
    for piece in &loop_points {
        curves.push(fit(piece)?);
    }
    let low = mirror(&[-3.0, start[1]]);
    curves.push(make_line(place(low, s0()), place([-3.0, start[1]], s0()))?);
    let (near, far) = (20.0, 38.0);
    let section = |at: f64| curves.iter().map(|c| scaled(c, at / s0())).collect::<Vec<_>>();
    let space = loft_profile_brep(&[section(near), section(far)])?;
    // The blank's meridian: s·g + u·d at its four corners (cone distance s, depth u from the pitch cone).
    let (g, d, _) = frame();
    let at = |s: f64, u: f64| g.scale(s).add(d.scale(u));
    let (outer, inner) = (s0(), s0() - 10.0);
    let meridian = [at(outer, -2.0), at(outer, 6.0), at(inner, 6.0), at(inner, -2.0 * inner / outer)];
    let blank = revolve_profile_brep(&polygon(&meridian)?, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)?;
    Ok(boolean_operation_nary(&[blank, space], BooleanOperation::Subtract)?)
}

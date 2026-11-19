//! What the gear-generation cases share: a tooth space cut by a tool under a generating motion,
//! found point by point from the set's own definition and built from the curves so found.
//!
//! A point is in the space where some place of the tool along the motion holds it: the case
//! gives the tool as inequalities and the motion as a map, so `Generating::margin(x, t)` is the
//! least slack of the tool's inequalities at the point x carried back by the motion at t
//! (positive inside). The space is cut by planes square to the gear's axis (slices); in each, it
//! is a star seen from a point Q outside the gear, so along the ray from Q at each angle β the
//! space (with the air beyond the radius `r_top`, outside the blank, counted in) runs from Q to
//! one distance D(β), found by bisection. Each ray's end lies on one surface of the tool's sweep:
//! the envelope of one of the tool's faces, or the track of one of its edges (where two
//! inequalities bind at once), or the circle r = r_top where the ray meets a tooth's top. Where
//! the binding inequalities change, the curve has a corner; corners are found by bisection, and
//! the curve between two is interpolated through its exact points by the kernel's cubic fit.
//!
//! Every slice must have the same corners in the same order; the slices' closed loops (Q, out
//! along the first ray, the curve, back along the last) are lofted by the kernel into the
//! space's cutter.
use super::p;
use crate::Refusal;
use brep_kernel::{interpolate_curve, loft_profile_brep, make_line, BrepSolid, NurbsCurve, Vec3};

/// A generating motion and its tool, from the case.
pub trait Generating {
    /// The least slack of the tool's inequalities at `x` carried back by the motion at `t`, and
    /// which of them bind (within `tight` of the least), as a bit set.
    fn margin(&self, x: Vec3, t: f64) -> (f64, u32);
    /// The motion parameters at which the tool can hold `x`: a range, and a step fine enough to
    /// see every local best of the margin in it.
    fn window(&self, x: Vec3) -> Option<(f64, f64, f64)>;
}

/// The bit set of the circle r = r_top, where a ray meets a tooth's top in the air.
pub const RIM: u32 = 1 << 31;

/// The best margin over the motion at `x`, and what binds there.
fn best<G: Generating>(gear: &G, x: Vec3) -> (f64, u32) {
    let Some((a, b, step)) = gear.window(x) else { return (-1.0, 0) };
    let n = ((b - a) / step).ceil().max(2.0) as usize;
    let samples: Vec<f64> = (0..=n).map(|i| gear.margin(x, a + (b - a) * i as f64 / n as f64).0).collect();
    // Refine round the three best samples.
    let mut order: Vec<usize> = (0..=n).collect();
    order.sort_by(|&i, &j| samples[j].total_cmp(&samples[i]));
    let mut top = (f64::NEG_INFINITY, 0);
    for &i in order.iter().take(3) {
        let lo = a + (b - a) * (i.max(1) - 1) as f64 / n as f64;
        let hi = a + (b - a) * (i + 1).min(n) as f64 / n as f64;
        let (mut l, mut h) = (lo, hi);
        let phi = 0.5 * (5f64.sqrt() - 1.0);
        let (mut c, mut d) = (h - phi * (h - l), l + phi * (h - l));
        let (mut fc, mut fd) = (gear.margin(x, c).0, gear.margin(x, d).0);
        for _ in 0..40 {
            if fc > fd {
                h = d;
                d = c;
                fd = fc;
                c = h - phi * (h - l);
                fc = gear.margin(x, c).0;
            } else {
                l = c;
                c = d;
                fc = fd;
                d = l + phi * (h - l);
                fd = gear.margin(x, d).0;
            }
        }
        let found = gear.margin(x, 0.5 * (l + h));
        if found.0 > top.0 {
            top = found;
        }
    }
    top
}

/// One slice of the space: the plane z = `z`, seen from `q` (in that plane), its rays at the
/// angles in [`from`, `to`] measured from the direction `toward` (a unit vector in the plane)
/// anticlockwise about +z.
pub struct Slice {
    pub z: f64,
    pub q: [f64; 2],
    pub toward: [f64; 2],
    pub from: f64,
    pub to: f64,
    pub r_top: f64,
    /// Farthest a ray is followed (past any part of the space).
    pub reach: f64,
}

impl Slice {
    fn ray(&self, beta: f64, s: f64) -> Vec3 {
        let (c, sn) = (beta.cos(), beta.sin());
        let u = [c * self.toward[0] - sn * self.toward[1], sn * self.toward[0] + c * self.toward[1]];
        p(self.q[0] + s * u[0], self.q[1] + s * u[1], self.z)
    }

    /// Whether the point at distance `s` along the ray at `beta` is in the space or in the air
    /// past r_top, and what bounds it there.
    fn inside<G: Generating>(&self, gear: &G, beta: f64, s: f64) -> (bool, u32) {
        let x = self.ray(beta, s);
        if x.x.hypot(x.y) > self.r_top {
            return (true, RIM);
        }
        let (m, bind) = best(gear, x);
        (m >= 0.0, bind)
    }

    /// The ray's end (distance D) and what binds there.
    fn end<G: Generating>(&self, gear: &G, beta: f64) -> Result<(f64, u32), Refusal> {
        // March out from Q to the first point outside, then bisect.
        let steps = ((self.reach / 0.25).ceil() as usize).max(8);
        let mut last = 0.0;
        for i in 1..=steps {
            let s = self.reach * i as f64 / steps as f64;
            if !self.inside(gear, beta, s).0 {
                let (mut a, mut b) = (last, s);
                for _ in 0..60 {
                    let m = 0.5 * (a + b);
                    if self.inside(gear, beta, m).0 {
                        a = m;
                    } else {
                        b = m;
                    }
                }
                // What binds at the boundary: the circle where the inside side is air, else
                // the tool's inequalities just inside it.
                let air = self.ray(beta, a);
                let bind = if air.x.hypot(air.y) > self.r_top - 1e-9 { RIM } else { best(gear, air).1 };
                return Ok((0.5 * (a + b), bind));
            }
            last = s;
        }
        Err(Refusal::from("a ray from outside the gear never leaves the tooth space"))
    }
}

/// One slice's corners: the angles where what binds changes, and the labels between them.
pub struct Corners {
    pub cuts: Vec<f64>,
    pub labels: Vec<u32>,
}

/// Find one slice's corners, from `samples` rays and bisection between them.
pub fn corners<G: Generating>(gear: &G, slice: &Slice, samples: usize) -> Result<Corners, Refusal> {
    let betas: Vec<f64> = (0..=samples).map(|i| slice.from + (slice.to - slice.from) * i as f64 / samples as f64).collect();
    let mut ends = Vec::with_capacity(betas.len());
    for &b in &betas {
        ends.push(slice.end(gear, b)?.1);
    }
    let mut cuts = vec![slice.from];
    let mut labels = vec![ends[0]];
    for i in 0..samples {
        if ends[i + 1] != ends[i] {
            let (mut a, mut b) = (betas[i], betas[i + 1]);
            for _ in 0..44 {
                let m = 0.5 * (a + b);
                if slice.end(gear, m)?.1 == ends[i] {
                    a = m;
                } else {
                    b = m;
                }
            }
            cuts.push(0.5 * (a + b));
            labels.push(ends[i + 1]);
        }
    }
    cuts.push(slice.to);
    Ok(Corners { cuts, labels })
}

/// The pieces every slice must have, in order. A slice whose pieces differ (one shrinking away
/// between slices) is refused: joining it to a neighbour would put a corner inside a fitted
/// curve, so the slices are to be taken where the pieces hold.
pub fn common(all: &[Corners]) -> Result<Vec<u32>, Refusal> {
    let first = &all[0].labels;
    for c in all {
        if &c.labels != first {
            return Err(Refusal::from(format!(
                "the tooth space's boundary changes its pieces between slices ({first:?} against {:?})",
                c.labels
            )));
        }
    }
    Ok(first.clone())
}

/// The loop of one slice sampled at `points` exact points per piece.
pub struct Traced {
    pub pieces: Vec<Vec<Vec3>>,
    pub q: Vec3,
}

pub fn sample<G: Generating>(gear: &G, slice: &Slice, corners: &Corners, kept: &[u32], points: usize) -> Result<Traced, Refusal> {
    let cuts = &corners.cuts;
    debug_assert_eq!(cuts.len(), kept.len() + 1);
    let mut pieces = Vec::new();
    for k in 0..kept.len() {
        let (a, b) = (cuts[k], cuts[k + 1]);
        let mut piece = Vec::with_capacity(points);
        for j in 0..points {
            let beta = a + (b - a) * j as f64 / (points - 1) as f64;
            let (d, _) = slice.end(gear, beta)?;
            piece.push(slice.ray(beta, d));
        }
        pieces.push(piece);
    }
    for k in 1..pieces.len() {
        let shared = pieces[k - 1][points - 1];
        pieces[k][0] = shared;
    }
    Ok(Traced { pieces, q: p(slice.q[0], slice.q[1], slice.z) })
}

fn fit(points: &[Vec3]) -> Result<NurbsCurve, Refusal> {
    let n = points.len();
    let parameters: Vec<f64> = (0..n).map(|i| i as f64 / (n - 1) as f64).collect();
    Ok(interpolate_curve(points, 3, &parameters)?)
}

/// The cutter: the slices' loops (Q, out along the first ray, the pieces, back along the last
/// ray) lofted.
pub fn cutter(slices: &[Traced]) -> Result<BrepSolid, Refusal> {
    let mut sections = Vec::new();
    for s in slices {
        let first = s.pieces[0][0];
        let last = *s.pieces.last().unwrap().last().unwrap();
        let mut curves = vec![make_line(s.q, first)?];
        for piece in &s.pieces {
            curves.push(fit(piece)?);
        }
        curves.push(make_line(last, s.q)?);
        sections.push(curves);
    }
    Ok(loft_profile_brep(&sections)?)
}

/// Trace the space in every slice and loft it.
pub fn space<G: Generating + Sync>(gear: &G, slices: &[Slice], samples: usize, points: usize) -> Result<BrepSolid, Refusal> {
    // The slices are independent: traced on as many threads as there are cores.
    let all: Vec<Corners> = each(slices, |slice| corners(gear, slice, samples))?;
    let kept = common(&all)?;
    let pairs: Vec<(&Slice, &Corners)> = slices.iter().zip(&all).collect();
    let traced: Vec<Traced> = each(&pairs, |(slice, c)| sample(gear, slice, c, &kept, points))?;
    cutter(&traced)
}

/// `f` of every item, in order, on as many threads as there are cores.
fn each<T: Sync, R: Send>(items: &[T], f: impl Fn(&T) -> Result<R, Refusal> + Sync) -> Result<Vec<R>, Refusal> {
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).min(items.len()).max(1);
    let mut out: Vec<Option<Result<R, Refusal>>> = (0..items.len()).map(|_| None).collect();
    std::thread::scope(|scope| {
        let chunks: Vec<_> = out.chunks_mut(items.len().div_ceil(threads)).zip(items.chunks(items.len().div_ceil(threads))).collect();
        for (slots, part) in chunks {
            let f = &f;
            scope.spawn(move || {
                for (slot, item) in slots.iter_mut().zip(part) {
                    *slot = Some(f(item));
                }
            });
        }
    });
    out.into_iter().map(|r| r.expect("every item is done")).collect()
}

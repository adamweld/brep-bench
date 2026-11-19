//! Two slabs united along the plane z = 4, where each has a face that is a B-spline patch lying
//! flat in it, the two parametrised differently: the box from (0, 0, 0) to (10, 10, 4), its top
//! the bicubic Bézier patch on (xᵢ, yⱼ, 4), xᵢ = 0, 2, 7, 10 and yⱼ = 0, 4, 5, 10; and the box
//! from (5, 3, 4) to (15, 13, 7), its bottom the biquadratic one on xᵢ = 5, 12, 15 and
//! yⱼ = 3, 4, 13. Each is sewn from its faces: the patch, and five planes, each upright side
//! bounded by the curve the patch's edge draws in it (the B-spline of that edge's control points).
use crate::bench::fuse;
use crate::shared::nurbs::{bezier_knots, graph_block, polygon, segment, sewn, spline, spline_curve, Face};
use crate::Refusal;
use solidcore_ops::Body;

/// The slab from the patch on (xs[i], ys[j], `z`), a Bézier patch each way, up to `top`.
fn slab_on(xs: &[f64], ys: &[f64], z: f64, top: f64) -> Result<Body, Refusal> {
    let knots = bezier_knots(xs.len() - 1);
    let net: Vec<Vec<[f64; 3]>> = xs.iter().map(|&x| ys.iter().map(|&y| [x, y, z]).collect()).collect();
    let (n, m) = (xs.len() - 1, ys.len() - 1);
    let up = |i: usize, j: usize| [xs[i], ys[j], top];
    // the side above the patch's edge `edge`, whose first and last points stand under `a` and `b`
    let side = |edge: Vec<[f64; 3]>, a: [f64; 3], b: [f64; 3]| {
        let (first, last) = (edge[0], edge[edge.len() - 1]);
        Face::Flat(vec![segment(a, b), segment(b, last), spline_curve(&knots, &edge), segment(first, a)])
    };
    sewn(&[
        Face::Surface(spline(&knots, &knots, net.clone())),
        Face::Flat(polygon(&[up(0, 0), up(n, 0), up(n, m), up(0, m)])),
        side(net[0].clone(), up(0, 0), up(0, m)),
        side(net[n].clone(), up(n, 0), up(n, m)),
        side(net.iter().map(|r| r[0]).collect(), up(0, 0), up(n, 0)),
        side(net.iter().map(|r| r[m]).collect(), up(0, m), up(n, m)),
    ])
}

pub fn build() -> Result<Body, Refusal> {
    let (xs, ys) = ([0.0, 2.0, 7.0, 10.0], [0.0, 4.0, 5.0, 10.0]);
    let lower = graph_block(&xs, &ys, &vec![vec![4.0; 4]; 4], &bezier_knots(3), &bezier_knots(3))?;
    let upper = slab_on(&[5.0, 12.0, 15.0], &[3.0, 4.0, 13.0], 4.0, 7.0)?;
    fuse(lower, upper)
}

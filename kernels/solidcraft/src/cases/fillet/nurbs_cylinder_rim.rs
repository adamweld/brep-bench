//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, its side one NURBS surface,
//! rational quadratic round the axis (u, the circle's nine control points 5cᵢ and weights wᵢ)
//! and linear up it (v); its ends flat discs inside the circles at z = 0 and z = 10. The three
//! faces, sewn into a solid, and the top rim rounded to radius 1.
//!
//! On solidcore-ops: SolidCraft's kernel makes a face on a NURBS net given as data through a
//! T-spline sheet (`tspline_sheet`), which takes odd degrees only, and this side is quadratic
//! round the axis.
use crate::bench::{edge_through, reloaded};
use crate::shared::nurbs::{bezier_knots, rational_curve, rational_spline, sewn, Face};
use crate::Refusal;
use solidcore_ops::{blend, Blend, Body, Corners};
use std::f64::consts::FRAC_1_SQRT_2;

/// The unit circle's control points cᵢ.
const C: [[f64; 2]; 9] =
    [[1.0, 0.0], [1.0, 1.0], [0.0, 1.0], [-1.0, 1.0], [-1.0, 0.0], [-1.0, -1.0], [0.0, -1.0], [1.0, -1.0], [1.0, 0.0]];
const CIRCLE_KNOTS: [f64; 12] = [0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0];

pub fn build() -> Result<Body, Refusal> {
    let w = |i: usize| if i % 2 == 1 { FRAC_1_SQRT_2 } else { 1.0 };
    let rim = |z: f64| -> Vec<[f64; 4]> { (0..9).map(|i| [5.0 * C[i][0], 5.0 * C[i][1], z, w(i)]).collect() };
    let (bottom, top) = (rim(0.0), rim(10.0));
    let net: Vec<Vec<[f64; 4]>> = (0..9).map(|i| vec![bottom[i], top[i]]).collect();
    let cylinder = sewn(&[
        Face::Surface(rational_spline(&CIRCLE_KNOTS, &bezier_knots(1), &net)),
        Face::Flat(vec![rational_curve(&CIRCLE_KNOTS, &bottom)]),
        Face::Flat(vec![rational_curve(&CIRCLE_KNOTS, &top)]),
    ])?;
    // the top rim, picked opposite the seam
    let (cylinder, order) = reloaded(&cylinder)?;
    let edge = edge_through(&cylinder, &order, [-5.0, 0.0, 10.0])?;
    blend(&cylinder, Blend::Round, &[edge], &[1.0], &[], Corners::Filled)?;
    Ok(cylinder)
}

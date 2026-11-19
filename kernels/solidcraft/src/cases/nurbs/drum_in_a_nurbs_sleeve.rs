//! A drum, the cylinder of radius 5 on the z axis from z = 0 to 10, united with a sleeve about it
//! from z = 2 to 8 whose bore is the drum's own cylinder as a NURBS surface.
//!
//! The sleeve is the cylinder of radius 7 less a slug sewn from its faces: the bore, rational
//! quadratic round the axis on the circle of radius 5's nine control points (from (0, 5), its
//! seam) and linear along it from z = 2 to 8, and the two flat discs inside its rims.
use crate::bench::{cut, fuse};
use crate::shared::nurbs::{bezier_knots, rational_curve, rational_spline, sewn, Face};
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};
use std::f64::consts::FRAC_1_SQRT_2;

/// The bore's control points round the axis, (x, y).
const C: [[f64; 2]; 9] =
    [[0.0, 5.0], [-5.0, 5.0], [-5.0, 0.0], [-5.0, -5.0], [0.0, -5.0], [5.0, -5.0], [5.0, 0.0], [5.0, 5.0], [0.0, 5.0]];
const CIRCLE_KNOTS: [f64; 12] = [0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0];

pub fn build() -> Result<Body, Refusal> {
    let w = |i: usize| if i % 2 == 1 { FRAC_1_SQRT_2 } else { 1.0 };
    let rim = |z: f64| -> Vec<[f64; 4]> { (0..9).map(|i| [C[i][0], C[i][1], z, w(i)]).collect() };
    let (low, high) = (rim(2.0), rim(8.0));
    let net: Vec<Vec<[f64; 4]>> = (0..9).map(|i| vec![low[i], high[i]]).collect();
    let slug = sewn(&[
        Face::Surface(rational_spline(&CIRCLE_KNOTS, &bezier_knots(1), &net)),
        Face::Flat(vec![rational_curve(&CIRCLE_KNOTS, &low)]),
        Face::Flat(vec![rational_curve(&CIRCLE_KNOTS, &high)]),
    ])?;
    let sleeve = cut(make_cylinder([0.0, 0.0, 2.0], 7.0, 6.0)?, slug)?;
    fuse(make_cylinder([0.0; 3], 5.0, 10.0)?, sleeve)
}

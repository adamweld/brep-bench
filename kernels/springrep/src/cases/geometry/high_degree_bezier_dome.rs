//! A block on the square 0 ≤ x, y ≤ 20 under a non-rational Bézier patch of degree 15 each way,
//! its control point (i, j) at height 4 + ((3i + 5j) mod 7). springrep's slab is exactly that: the
//! solid under a clamped B-spline height field whose coefficients sit at the Greville abscissae,
//! which for a Bézier of degree 15 are i/15, the control net's even grid.
use springrep::primitive::slab;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // a single Bézier span: sixteen zeros and sixteen ones
    let knots: Vec<f64> = [vec![0.; 16], vec![1.; 16]].concat();
    let heights: Vec<Vec<f64>> = (0..16)
        .map(|i| (0..16).map(|j| 4. + ((3 * i + 5 * j) % 7) as f64).collect())
        .collect();
    slab(&at([0., 0., 0.]), [20., 20.], (&knots, &knots), &heights, "dome").map_err(e)
}

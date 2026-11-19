//! The drum ρ ≤ 8, 0 ≤ z ≤ 10 + xy/8 under a saddle. The saddle is the bilinear B-spline patch over
//! −10 ≤ x, y ≤ 10 with its corners at heights 22.5, −2.5, −2.5, 22.5: springrep's slab, the solid
//! under a clamped B-spline height field, here of degree 1 on a 2 × 2 net. A slab's field must
//! stand above its floor, so the slab's floor is put at z = −5; the drum is its common part with
//! the cylinder of radius 8 from z = 0 up past the saddle, which gives the floor at z = 0.
use springrep::boolean::intersection;
use springrep::primitive::{cylinder, slab};
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let floor = -5.;
    let knots = [0., 0., 1., 1.];
    // heights above the slab's floor, at the corners (x, y) = (∓10, ∓10)
    let heights = vec![
        vec![22.5 - floor, -2.5 - floor],
        vec![-2.5 - floor, 22.5 - floor],
    ];
    let saddle = slab(&at([-10., -10., floor]), [20., 20.], (&knots, &knots), &heights, "saddle")
        .map_err(e)?;
    let drum = cylinder(&at([0., 0., 0.]), 8., 30., "drum").map_err(e)?;
    intersection(&saddle, &drum).map_err(e)
}

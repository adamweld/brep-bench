//! The block under a bicubic Bézier dome over the square from (0, 0) to (20, 20), the edge where
//! the dome meets the side y = 0 rounded to radius 1 by a rolling ball.
use springrep::blend::fillet;
use springrep::primitive::slab;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // a slab's heights are a clamped B-spline's coefficients at the knots' Greville abscissae: on
    // the cubic Bézier knots those are 0, 1/3, 2/3 and 1, so x = 20i/3 and y = 20j/3
    let bezier = [0., 0., 0., 0., 1., 1., 1., 1.];
    let heights = vec![
        vec![4., 6., 7., 5.],
        vec![6., 14., 12., 7.],
        vec![5., 13., 16., 8.],
        vec![3., 6., 8., 6.],
    ];
    let dome = slab(&at([0., 0., 0.]), [20., 20.], (&bezier, &bezier), &heights, "dome").map_err(e)?;
    // the slab's side y = 0 is its `front`
    fillet(&dome, &[("dome.top", "dome.front")], 1., "round").map_err(e)
}

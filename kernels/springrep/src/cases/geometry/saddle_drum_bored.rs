//! The saddle drum of `geometry/saddle-drum` (ρ ≤ 8, 0 ≤ z ≤ 10 + xy/8, its top the bilinear
//! B-spline patch as springrep's slab) bored through by a cylinder of radius 3 running past the
//! floor and the saddle.
use springrep::boolean::{difference, intersection};
use springrep::primitive::{cylinder, slab};
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let floor = -5.;
    let knots = [0., 0., 1., 1.];
    let heights = vec![
        vec![22.5 - floor, -2.5 - floor],
        vec![-2.5 - floor, 22.5 - floor],
    ];
    let saddle = slab(&at([-10., -10., floor]), [20., 20.], (&knots, &knots), &heights, "saddle")
        .map_err(e)?;
    let drum = cylinder(&at([0., 0., 0.]), 8., 30., "drum").map_err(e)?;
    let drum = intersection(&saddle, &drum).map_err(e)?;
    let bore = cylinder(&at([0., 0., -1.]), 3., 20., "bore").map_err(e)?;
    difference(&drum, &bore).map_err(e)
}

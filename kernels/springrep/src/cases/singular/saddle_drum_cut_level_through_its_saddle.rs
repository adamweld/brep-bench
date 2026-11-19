//! The drum of radius 8 on the z axis, floor z = 0, under the bilinear saddle z = 10 + xy/8 over
//! the square −10 ≤ x, y ≤ 10, cut level at z = 10 through the saddle point.
use springrep::boolean::intersection;
use springrep::primitive::{cylinder, slab};
use springrep::topo::Brep;

use crate::shared::singular::cut_away;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // The saddle is a slab's top: the degree-1 B-spline over the square on its four corners. A
    // slab's heights must stand above its floor, and the saddle falls to −2.5 at two corners, so
    // the slab's floor is at z = −5 and the drum's own floor comes from the cylinder.
    let linear = [0., 0., 1., 1.];
    let corner = |x: f64, y: f64| 10. + x * y / 8. + 5.;
    let heights = vec![
        vec![corner(-10., -10.), corner(-10., 10.)],
        vec![corner(10., -10.), corner(10., 10.)],
    ];
    let saddle = slab(&at([-10., -10., -5.]), [20., 20.], (&linear, &linear), &heights, "saddle")
        .map_err(e)?;
    let drum = cylinder(&at([0., 0., 0.]), 8., 30., "drum").map_err(e)?;
    let under = intersection(&saddle, &drum).map_err(e)?;
    cut_away(&under, [0., 0., 10.], [0., 0., 1.])
}

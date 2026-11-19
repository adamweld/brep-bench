//! A drum of radius 5 on the z axis from z = 0 to 10, united with a sleeve from radius 5 to 7 and
//! z = 2 to 8 whose bore is the drum's cylinder written as a rational B-spline.
use std::f64::consts::FRAC_1_SQRT_2;

use springrep::boolean::union;
use springrep::math::Vec2;
use springrep::primitive::cylinder;
use springrep::profile::{NamedSegment, Profile, ProfileLoop, Segment};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let drum = cylinder(&at([0., 0., 0.]), 5., 10., "drum").map_err(e)?;
    // The bore's circle as the rational quadratic B-spline of nine points with double knots at
    // the quarters: four weighted Bézier arcs from (0, 5) round through (−5, 0). Extruded, each
    // is the degree (2, 1) rational surface of the case.
    let ring = [(0., 5.), (-5., 5.), (-5., 0.), (-5., -5.), (0., -5.), (5., -5.), (5., 0.), (5., 5.)];
    let p = |i: usize| Vec2::new(ring[i % 8].0, ring[i % 8].1);
    let bore = (0..4)
        .map(|k| {
            let arc = Segment::Bezier {
                ctrl: vec![p(2 * k), p(2 * k + 1), p(2 * k + 2)],
                weights: vec![1., FRAC_1_SQRT_2, 1.],
            };
            NamedSegment::new(arc, "bore")
        })
        .collect();
    let sleeve = Profile::named_circle(Vec2::ZERO, 14., "outside")
        .with_holes([ProfileLoop { segments: bore }]);
    let sleeve = extrude(&sleeve, &at([0., 0., 2.]), 6., "sleeve").map_err(e)?;
    union(&drum, &sleeve).map_err(e)
}

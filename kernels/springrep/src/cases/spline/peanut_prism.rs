//! A prism 10 tall on the xy plane whose section is the peanut inside a closed uniform cubic
//! B-spline of twelve control points.
use springrep::geom::BSpline;
use springrep::math::Vec3;
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let poles = [
        (14., 0.), (12., 8.), (4., 6.), (0., -21. / 8.), (-4., 6.), (-12., 8.),
        (-14., 0.), (-12., -8.), (-4., -6.), (0., 21. / 8.), (4., -6.), (12., -8.),
    ];
    // a periodic cubic takes its n control points with the first three again, and n + 7 uniform
    // knots, three before the period and three after it
    let mut ctrl: Vec<Vec3> = poles.iter().map(|&(x, y)| Vec3::new(x, y, 0.)).collect();
    ctrl.extend_from_slice(&ctrl[..3].to_vec());
    let knots = (0..poles.len() + 7).map(|i| i as f64 - 3.).collect();
    let curve = BSpline::periodic_cubic(knots, ctrl).map_err(e)?;
    let outline = Segment::Spline { curve, reversed: false };
    let section = Profile::outline(vec![NamedSegment::new(outline, "side")]);
    extrude(&section, &at([0., 0., 0.]), 10., "peanut").map_err(e)
}

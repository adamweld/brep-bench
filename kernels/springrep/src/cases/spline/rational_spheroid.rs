//! A spheroid turned about the z axis from the region between the axis and the weighted
//! quadratic B-spline (0, 0, −10), (6, 0, −10), (6, 0, 0), (6, 0, 10), (0, 0, 10), weights
//! 1, √2/2, 1, √2/2, 1, its double knot at ½.
use std::f64::consts::{FRAC_1_SQRT_2, TAU};

use springrep::math::{Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    let w = FRAC_1_SQRT_2;
    // the double knot splits the B-spline into two weighted Bézier arcs, a profile's segments
    let quarter = |a: (f64, f64), b: (f64, f64), c: (f64, f64)| Segment::Bezier {
        ctrl: vec![Vec2::new(a.0, a.1), Vec2::new(b.0, b.1), Vec2::new(c.0, c.1)],
        weights: vec![1., w, 1.],
    };
    let region = Profile::outline(vec![
        NamedSegment::new(quarter((0., -10.), (6., -10.), (6., 0.)), "skin"),
        NamedSegment::new(quarter((6., 0.), (6., 10.), (0., 10.)), "skin"),
        NamedSegment::line(Vec2::new(0., 10.), Vec2::new(0., -10.), "axis"),
    ]);
    revolve(&region, &xz, Vec3::ZERO, Vec3::Z, TAU, "spheroid").map_err(e)
}

//! Helpers shared by the geometry cases.

use springrep::geom::BSpline;
use springrep::math::{Vec2, Vec3};
use springrep::profile::{NamedSegment, Segment};
use std::f64::consts::PI;

use crate::shared::e;

/// The whole ellipse about `center` with semi-axes `major` along x and `minor` along y, counter-
/// clockwise, as its two halves: a profile loop of one segment must be a closed curve, and
/// springrep counts only a circle or a periodic spline as one.
pub fn ellipse(center: Vec2, major: f64, minor: f64, name: &str) -> Vec<NamedSegment> {
    [0., PI]
        .map(|start| {
            let half = Segment::Ellipse {
                center,
                rotation: 0.,
                major,
                minor,
                start,
                sweep: PI,
            };
            NamedSegment::new(half, name)
        })
        .to_vec()
}

/// The closed uniform quadratic B-spline on the control polygon `corners` (four of them, in
/// order), as two open splines of the same curve on the same uniform knots, each over two of its
/// four spans: springrep's only periodic spline is a cubic, and a profile loop of one open spline
/// is refused, so the curve is the two pieces, joined with one tangent at the middles of the first
/// and third sides.
pub fn closed_quadratic(corners: [[f64; 2]; 4], name: &str) -> Result<Vec<NamedSegment>, String> {
    let point = |i: usize| Vec3::new(corners[i % 4][0], corners[i % 4][1], 0.);
    [0, 2]
        .map(|first| {
            let ctrl = (first..first + 4).map(point).collect();
            let knots = (first..first + 7).map(|k| k as f64).collect();
            let curve = BSpline::new(2, knots, ctrl).map_err(e)?;
            let piece = Segment::Spline {
                curve,
                reversed: false,
            };
            Ok(NamedSegment::new(piece, name))
        })
        .into_iter()
        .collect()
}

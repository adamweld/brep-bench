//! The lens common to two balls of radius 5 about (−3, 0, 0) and (3, 0, 0), each bounded by one
//! rational surface whose meridian is the weighted quadratic semicircle, not by a sphere.
use std::f64::consts::{FRAC_1_SQRT_2, TAU};

use springrep::boolean::intersection;
use springrep::math::{Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::{e, frame};

/// The ball about `(cx, 0, 0)`: its meridian, the rational quadratic semicircle of five weighted
/// control points from the south pole to the north, turned about the upright axis through the
/// centre from the half plane y = 0 on the +x side, where its seam lies.
fn ball(cx: f64) -> Result<Brep, String> {
    let plane = frame([cx, 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    let p = |x: f64, z: f64| Vec2::new(x, z);
    let w = FRAC_1_SQRT_2;
    let quarter = |a, b, c| Segment::Bezier { ctrl: vec![a, b, c], weights: vec![1., w, 1.] };
    let meridian = Profile::outline(vec![
        NamedSegment::new(quarter(p(0., -5.), p(5., -5.), p(5., 0.)), "skin"),
        NamedSegment::new(quarter(p(5., 0.), p(5., 5.), p(0., 5.)), "skin"),
        NamedSegment::line(p(0., 5.), p(0., -5.), "axis"),
    ]);
    revolve(&meridian, &plane, Vec3::new(cx, 0., 0.), Vec3::Z, TAU, "ball").map_err(e)
}

pub fn build() -> Result<Brep, String> {
    intersection(&ball(-3.)?, &ball(3.)?).map_err(e)
}

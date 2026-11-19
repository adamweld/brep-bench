//! A ball's cap turned about the z axis from the region bounded by the axis, the segment
//! (0, 0, −5) → (5√3, 0, −5) and the weighted quadratic Bézier (5√3, 0, −5), (10√3, 0, 10),
//! (0, 0, 10) with weights 1, ½, 1.
use std::f64::consts::TAU;

use springrep::math::{Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    let s3 = 3f64.sqrt();
    let rim = Vec2::new(5. * s3, -5.);
    let arc = Segment::Bezier {
        ctrl: vec![rim, Vec2::new(10. * s3, 10.), Vec2::new(0., 10.)],
        weights: vec![1., 0.5, 1.],
    };
    let region = Profile::outline(vec![
        NamedSegment::line(Vec2::new(0., -5.), rim, "base"),
        NamedSegment::new(arc, "dome"),
        NamedSegment::line(Vec2::new(0., 10.), Vec2::new(0., -5.), "axis"),
    ]);
    revolve(&region, &xz, Vec3::ZERO, Vec3::Z, TAU, "cap").map_err(e)
}

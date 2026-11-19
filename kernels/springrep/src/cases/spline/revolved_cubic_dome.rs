//! A dome turned about the z axis from the region bounded by the axis, the x axis out to 9 and
//! the cubic Bézier (9, 0, 0), (12, 0, 4), (9, 0, 10), (0, 0, 10).
use std::f64::consts::TAU;

use springrep::math::{Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    let poles = [(9., 0.), (12., 4.), (9., 10.), (0., 10.)];
    let curve = Segment::Bezier {
        ctrl: poles.iter().map(|&(x, z)| Vec2::new(x, z)).collect(),
        weights: vec![1.; 4],
    };
    let region = Profile::outline(vec![
        NamedSegment::line(Vec2::ZERO, Vec2::new(9., 0.), "base"),
        NamedSegment::new(curve, "dome"),
        NamedSegment::line(Vec2::new(0., 10.), Vec2::ZERO, "axis"),
    ]);
    revolve(&region, &xz, Vec3::ZERO, Vec3::Z, TAU, "dome").map_err(e)
}

//! The oblate spheroid (x² + y²)/36 + z²/9 ≤ 1: the half ellipse x²/36 + z²/9 ≤ 1, x ≥ 0, of the xz
//! plane turned a whole turn about the z axis, its short axis.
use springrep::math::{Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::revolve;
use springrep::topo::Brep;
use std::f64::consts::{FRAC_PI_2, PI, TAU};

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let arc = Segment::Ellipse {
        center: Vec2::ZERO,
        rotation: 0.,
        major: 6.,
        minor: 3.,
        start: -FRAC_PI_2,
        sweep: PI,
    };
    let half = Profile::outline(vec![
        NamedSegment::new(arc, "skin"),
        NamedSegment::line(Vec2::new(0., 3.), Vec2::new(0., -3.), "axis"),
    ]);
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    revolve(&half, &xz, Vec3::ZERO, Vec3::Z, TAU, "spheroid").map_err(e)
}

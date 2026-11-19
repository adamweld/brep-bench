//! A circle of radius 2 swept along a quarter of the circle of radius 10 about the z axis, from
//! (10, 0, 0) to (0, 10, 0), kept square to the path.
use std::f64::consts::FRAC_PI_2;

use springrep::math::{Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::{Leg, along};
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the section starts square to the path, heading along +y
    let start = frame([10., 0., 0.], [0., 1., 0.], [1., 0., 0.])?;
    let section = Profile::circle(Vec2::ZERO, 4.);
    // a bend about +z turns the heading +y toward z × y = −x, about the origin
    let bend = Leg::Arc { radius: 10., angle: FRAC_PI_2, axis: Vec3::Z };
    along(&section, &start, &[bend], "pipe").map_err(e)
}

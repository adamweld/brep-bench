//! A hollow square duct, 18 × 18 outside with a 14 × 14 hole, bent a quarter turn along the arc
//! of radius 30 about the z axis from (30, 0, 0) to (0, 30, 0).
use std::f64::consts::FRAC_PI_2;

use springrep::math::{Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::{Leg, along};
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let start = frame([30., 0., 0.], [0., 1., 0.], [1., 0., 0.])?;
    let hole = Profile::rect(Vec2::ZERO, 14., 14.).loops;
    let section = Profile::rect(Vec2::ZERO, 18., 18.).with_holes(hole);
    let bend = Leg::Arc { radius: 30., angle: FRAC_PI_2, axis: Vec3::Z };
    along(&section, &start, &[bend], "duct").map_err(e)
}

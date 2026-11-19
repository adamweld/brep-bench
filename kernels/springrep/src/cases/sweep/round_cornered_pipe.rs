//! A pipe of radius 3 along (−30, 0, 0) → (0, 0, 0) → (0, 30, 0), the sharp corner filled by the
//! section turned about the z axis through it.
use std::f64::consts::FRAC_PI_2;

use springrep::math::{Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::{Leg, along};
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let start = frame([-30., 0., 0.], [1., 0., 0.], [0., 0., 1.])?;
    let section = Profile::circle(Vec2::ZERO, 6.);
    let legs = [
        Leg::Line { length: 30. },
        // a turn on the spot fills the outside of the corner with the section turned about it
        Leg::Turn { angle: FRAC_PI_2, axis: Vec3::Z },
        Leg::Line { length: 30. },
    ];
    along(&section, &start, &legs, "pipe").map_err(e)
}

//! The box (−6, −6, −8)–(18, 18, 0) with a groove cut by a ball of radius 3 whose center swings a
//! quarter turn about the z axis from (12, 0, 0) to (0, 12, 0).
//!
//! springrep has no sweep of a solid. Every circle about the z axis meets the ball in one arc,
//! across the plane y = 0, so the ball's sweep is the ball where it starts and where it stops and
//! its section there, a disc of radius 3 about (12, 0), turned through the quarter turn: a quarter
//! of a torus (`shared::swept`, a turn).
use std::f64::consts::FRAC_PI_2;

use springrep::boolean::difference;
use springrep::primitive::sphere;
use springrep::profile::Profile;
use springrep::topo::Brep;

use crate::shared::swept::{boxed, swing, turned, unite, v2};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let block = boxed([-6.0, -6.0, -8.0], [18.0, 18.0, 0.0], "block")?;
    let ball = sphere(&at([12.0, 0.0, 0.0]), 3.0, "ball").map_err(e)?;
    let tube = swing(&Profile::circle(v2(12.0, 0.0), 6.0), 0.0, FRAC_PI_2, "groove")?;
    let groove = unite(&[&ball, &tube, &turned(&ball, FRAC_PI_2)])?;
    difference(&block, &groove).map_err(e)
}

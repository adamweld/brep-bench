//! A cylinder of radius 10 along the x axis with a square pad on top, the square |x|, |y| ≤ 3 at
//! z = 15 extruded down until it meets the cylinder's side, and joined to it.
//!
//! springrep has no extrusion up to a face, so the pad is extruded down past the cylinder's side
//! into the cylinder (to its axis), and the union leaves it ending where it meets the side.
use springrep::boolean::union;
use springrep::math::Vec2;
use springrep::primitive::cylinder;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    let along_x = frame([-20., 0., 0.], [1., 0., 0.], [0., 1., 0.])?;
    let body = cylinder(&along_x, 10., 40., "cyl").map_err(e)?;
    let square = Profile::rect(Vec2::ZERO, 6., 6.);
    let pad = extrude(&square, &at([0., 0., 15.]), -15., "pad").map_err(e)?;
    union(&body, &pad).map_err(e)
}

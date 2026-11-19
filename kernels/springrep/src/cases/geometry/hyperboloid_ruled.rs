//! The solid between two disks of radius 5 at z = −5 and z = 5 and the surface ruled between their
//! rims, each point at angle θ below joined straight to the one at θ + 90° above: a ruled loft of
//! the two circles, the upper one's start turned a quarter turn.
use springrep::loft::{Section, loft};
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let rim = Profile::circle(Vec2::ZERO, 10.);
    // each circle starts at its frame's x axis: along x below, along y above
    let below = Section {
        profile: rim.clone(),
        frame: frame([0., 0., -5.], [0., 0., 1.], [1., 0., 0.])?,
    };
    let above = Section {
        profile: rim,
        frame: frame([0., 0., 5.], [0., 0., 1.], [0., 1., 0.])?,
    };
    loft(&[below, above], true, "twist").map_err(e)
}

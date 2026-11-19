//! A blade, the rectangle 40 × 0.4 centred on the z axis, turned 6° per unit of height from z = 0
//! to 60: a whole turn, its corners on four helices.
use std::f64::consts::TAU;

use springrep::math::{Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::{Helix, helix};
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // A section square to the axis screwed along it is a twisted extrusion: the rectangle turned
    // right-handed about z, rising 60 a turn, its walls ruled between its corners' helices.
    let blade = Profile::rect(Vec2::ZERO, 40., 0.4);
    let screw = Helix { origin: Vec3::ZERO, axis: Vec3::Z, rise: 60. / TAU, sign: 1. };
    helix(&blade, &at([0., 0., 0.]), &screw, TAU, "blade").map_err(e)
}

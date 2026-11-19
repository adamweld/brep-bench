//! The cylinder of radius 0.00075 and height 0.0015 about the z axis from z = 0 and a copy of it
//! slid 0.000015 along x, a fiftieth of its radius: the cylinder less the copy.
use springrep::boolean::difference;
use springrep::math::{Transform, Vec3};
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let small = cylinder(&at([0., 0., 0.]), 0.00075, 0.0015, "cylinder").map_err(e)?;
    let mut slid = small.clone();
    slid.transform(&Transform::translation(Vec3::new(0.000015, 0., 0.)));
    difference(&small, &slid).map_err(e)
}

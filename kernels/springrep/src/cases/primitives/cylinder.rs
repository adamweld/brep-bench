//! A cylinder of radius 5 about the z axis, from z = 0 to z = 10.
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    cylinder(&at([0., 0., 0.]), 5., 10., "cylinder").map_err(e)
}

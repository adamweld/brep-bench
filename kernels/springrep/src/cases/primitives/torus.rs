//! A torus about the z axis, its tube's centre circle of radius 10 and its tube of radius 2.
use springrep::primitive::torus;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    torus(&at([0., 0., 0.]), 10., 2., "ring").map_err(e)
}

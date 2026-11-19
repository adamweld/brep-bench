//! A ball of radius 5 about the origin.
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)
}

//! The lens common to two balls of radius 5, about the origin and about (9.999999, 0, 0): 10⁻⁶
//! thick at its middle.
use springrep::boolean::intersection;
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let a = sphere(&at([0., 0., 0.]), 5., "a").map_err(e)?;
    let b = sphere(&at([9.999999, 0., 0.]), 5., "b").map_err(e)?;
    intersection(&a, &b).map_err(e)
}

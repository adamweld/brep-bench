//! The part common to two tori about the z axis, of middle radius 5 and tube radius 2, their middle
//! circles in z = -1 and z = 1.
use springrep::boolean::intersection;
use springrep::primitive::torus;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let lower = torus(&at([0., 0., -1.]), 5., 2., "lower").map_err(e)?;
    let upper = torus(&at([0., 0., 1.]), 5., 2., "upper").map_err(e)?;
    intersection(&lower, &upper).map_err(e)
}

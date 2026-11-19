//! The torus about the z axis whose middle circle has radius 5 and whose tube has radius 4.99,
//! leaving a hole 0.02 across.
use springrep::primitive::torus;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    torus(&at([0., 0., 0.]), 5., 4.99, "ring").map_err(e)
}

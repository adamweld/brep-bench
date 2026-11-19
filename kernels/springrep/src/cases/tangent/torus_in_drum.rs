//! The torus about z (middle radius 10, tube radius 3) united with the drum of radius 13 from
//! z = -1 to 1, whose wall it touches from inside along its outer equator.
use springrep::boolean::union;
use springrep::primitive::{cylinder, torus};
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ring = torus(&at([0., 0., 0.]), 10., 3., "torus").map_err(e)?;
    let drum = cylinder(&at([0., 0., -1.]), 13., 2., "drum").map_err(e)?;
    union(&ring, &drum).map_err(e)
}

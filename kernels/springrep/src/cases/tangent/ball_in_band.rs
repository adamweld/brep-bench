//! The ball of radius 5 about the origin united with the cylinder of radius 5 on the z axis from
//! z = -3 to 3, which it touches from inside along the equator.
use springrep::boolean::union;
use springrep::primitive::{cylinder, sphere};
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)?;
    let band = cylinder(&at([0., 0., -3.]), 5., 6., "band").map_err(e)?;
    union(&ball, &band).map_err(e)
}

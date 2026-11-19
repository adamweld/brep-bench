//! The cylinder of radius 5 from z = 0 to 10 less the ball of radius 5 about the middle of its top:
//! a hemispherical dish meeting the wall tangentially in a knife edge.
use springrep::boolean::difference;
use springrep::primitive::{cylinder, sphere};
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let stock = cylinder(&at([0., 0., 0.]), 5., 10., "stock").map_err(e)?;
    let ball = sphere(&at([0., 0., 10.]), 5., "dish").map_err(e)?;
    difference(&stock, &ball).map_err(e)
}

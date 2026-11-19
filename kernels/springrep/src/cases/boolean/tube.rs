//! A tube: the cylinder of radius 5 from z = 0 to 10 bored along its axis by a hole of radius 3.
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let outer = cylinder(&at([0., 0., 0.]), 5., 10., "outer").map_err(e)?;
    // the drill runs past both ends, from z = -1 to 11
    let drill = cylinder(&at([0., 0., -1.]), 3., 12., "bore").map_err(e)?;
    difference(&outer, &drill).map_err(e)
}

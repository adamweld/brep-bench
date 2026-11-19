//! The cylinder of radius 5 from z = 0 to 10 bored along z by a hole of radius 4 about the line
//! x = 0.999999, y = 0, which leaves a wall 10⁻⁶ thick at (5, 0, z).
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let stock = cylinder(&at([0., 0., 0.]), 5., 10., "stock").map_err(e)?;
    // the drill runs past both ends, from z = -1 to 11
    let drill = cylinder(&at([0.999999, 0., -1.]), 4., 12., "bore").map_err(e)?;
    difference(&stock, &drill).map_err(e)
}

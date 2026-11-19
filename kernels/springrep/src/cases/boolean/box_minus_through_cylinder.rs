//! The cube [0, 10]³ drilled through along z by a hole of radius 3 about x = 5, y = 5.
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::boolean::between;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let cube = between([0., 0., 0.], [10., 10., 10.], "cube")?;
    // the drill runs past both faces, from z = -1 to 11
    let drill = cylinder(&at([5., 5., -1.]), 3., 12., "hole").map_err(e)?;
    difference(&cube, &drill).map_err(e)
}

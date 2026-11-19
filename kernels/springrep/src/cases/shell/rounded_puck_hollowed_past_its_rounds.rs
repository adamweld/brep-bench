//! A cylinder of radius 6 from z = 0 to z = 10, both its circular edges rounded to radius 2,
//! hollowed to a wall 3 thick with no opening. The rounds moved 3 inward turn inside out and drop:
//! the cavity is the cylinder of radius 3 from z = 3 to z = 7.
use springrep::blend::fillet;
use springrep::edit::Wall;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let puck = cylinder(&at([0., 0., 0.]), 6., 10., "puck").map_err(e)?;
    let rims = [("puck.side", "puck.start"), ("puck.side", "puck.end")];
    let rounded = fillet(&puck, &rims, 2., "round").map_err(e)?;
    shelled(&rounded, &[], 3., Wall::Inward, "shell")
}

//! A cylinder of radius 6 from z = 0 to z = 10, its bottom edge rounded to radius 2, shelled inward
//! to a wall 3 thick, open at its top. The round moved 3 inward turns inside out and drops: the
//! cavity is the cylinder of radius 3 from z = 3 up through the top.
use springrep::blend::fillet;
use springrep::edit::Wall;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let puck = cylinder(&at([0., 0., 0.]), 6., 10., "puck").map_err(e)?;
    let rounded = fillet(&puck, &[("puck.side", "puck.start")], 2., "round").map_err(e)?;
    shelled(&rounded, &["puck.end"], 3., Wall::Inward, "shell")
}

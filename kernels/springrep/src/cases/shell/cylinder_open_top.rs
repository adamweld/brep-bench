//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, shelled inward to a wall 1 thick, open
//! at its top: a cup.
use springrep::edit::Wall;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // its faces are `start` (z = 0), `end` (z = 10) and `side`
    let c = cylinder(&at([0., 0., 0.]), 5., 10., "can").map_err(e)?;
    shelled(&c, &["can.end"], 1., Wall::Inward, "shell")
}

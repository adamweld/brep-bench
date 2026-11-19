//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, shelled inward to a wall 1 thick, open
//! at its curved side: two disks 1 thick, its ends.
use springrep::edit::Wall;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let c = cylinder(&at([0., 0., 0.]), 5., 10., "can").map_err(e)?;
    // only the ends are walls; nothing joins them, so the result is two solids
    shelled(&c, &["can.side"], 1., Wall::Inward, "shell")
}

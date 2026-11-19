//! A cone frustum on the z axis, radius 6 at z = 0 to 3 at z = 4, bored through by a hole of radius
//! 1, shelled inward to a wall 1 thick, open at its top. The moved side meets the moved bore below
//! the top, so the cavity is a closed ring and the top stays.
use springrep::boolean::difference;
use springrep::edit::Wall;
use springrep::primitive::{cone, cylinder};
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let frustum = cone(&at([0., 0., 0.]), 6., 3., 4., "frustum").map_err(e)?;
    let hole = cylinder(&at([0., 0., -1.]), 1., 6., "bore").map_err(e)?;
    let bored = difference(&frustum, &hole).map_err(e)?;
    // the top, the annulus between radii 1 and 3, keeps its name
    shelled(&bored, &["frustum.top"], 1., Wall::Inward, "shell")
}

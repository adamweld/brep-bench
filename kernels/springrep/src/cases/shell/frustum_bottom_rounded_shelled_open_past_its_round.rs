//! A cone frustum on the z axis, radius 6 at z = 0 to 3 at z = 6, its bottom rim rounded to radius
//! 1.5, shelled inward to a wall 2 thick, open at its top. The round moved inward turns inside out
//! and drops, and the cavity has the sharp frustum's corner.
use springrep::blend::fillet;
use springrep::edit::Wall;
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let f = cone(&at([0., 0., 0.]), 6., 3., 6., "frustum").map_err(e)?;
    let rounded = fillet(&f, &[("frustum.side", "frustum.bottom")], 1.5, "round").map_err(e)?;
    shelled(&rounded, &["frustum.top"], 2., Wall::Inward, "shell")
}

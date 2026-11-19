//! A cone frustum on the z axis, radius 6 at z = 0 to 3 at z = 4, shelled inward to a wall 2.7 thick,
//! open at both ends. The moved side closes to its apex below the top, so only the base opens.
use springrep::edit::Wall;
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // a frustum's faces are `bottom`, `side` and `top`
    let f = cone(&at([0., 0., 0.]), 6., 3., 4., "frustum").map_err(e)?;
    shelled(&f, &["frustum.bottom", "frustum.top"], 2.7, Wall::Inward, "shell")
}

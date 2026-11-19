//! A cone frustum on the z axis, radius 7 at z = 0 to 1 at z = 8, shelled inward to a wall 2 thick,
//! open at its top. The moved side closes to its apex 2 below the top: the cavity is a closed cone.
use springrep::edit::Wall;
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // a frustum's faces are `bottom`, `side` and `top`
    let f = cone(&at([0., 0., 0.]), 7., 1., 8., "frustum").map_err(e)?;
    shelled(&f, &["frustum.top"], 2., Wall::Inward, "shell")
}

//! A cone frustum on the z axis, radius 6 at z = 0 to 3 at z = 6, both rims rounded to radius 1.5,
//! hollowed to a wall 2 thick with no opening. The rounds moved inward turn inside out and drop: the
//! cavity is the sharp frustum shrunk by the wall.
use springrep::blend::fillet;
use springrep::edit::Wall;
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let f = cone(&at([0., 0., 0.]), 6., 3., 6., "frustum").map_err(e)?;
    let rims = [("frustum.side", "frustum.bottom"), ("frustum.side", "frustum.top")];
    let rounded = fillet(&f, &rims, 1.5, "round").map_err(e)?;
    shelled(&rounded, &[], 2., Wall::Inward, "shell")
}

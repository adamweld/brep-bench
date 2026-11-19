//! A solid cone on the z axis, its base of radius 6 at z = 0 and its apex at (0, 0, 8), shelled
//! inward to a wall 1 thick, open at its base.
use springrep::edit::Wall;
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // a top radius of zero is an apex: the cone's faces are `bottom` and `side`
    let c = cone(&at([0., 0., 0.]), 6., 0., 8., "cone").map_err(e)?;
    shelled(&c, &["cone.bottom"], 1., Wall::Inward, "shell")
}

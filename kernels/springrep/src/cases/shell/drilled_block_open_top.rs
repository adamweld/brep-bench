//! The block from the origin to (20, 20, 10) bored through along z by a hole of radius 3 about
//! x = 10, y = 10, shelled inward to a wall 1 thick, open at its top: a cup with a tube standing in it.
use springrep::boolean::difference;
use springrep::edit::Wall;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(20., 20., 10.), "block").map_err(e)?;
    let hole = cylinder(&at([10., 10., -1.]), 3., 12., "bore").map_err(e)?;
    let drilled = difference(&b, &hole).map_err(e)?;
    // the bore faces the axis: moved inward, into the material, it grows to radius 4
    shelled(&drilled, &["block.end"], 1., Wall::Inward, "shell")
}

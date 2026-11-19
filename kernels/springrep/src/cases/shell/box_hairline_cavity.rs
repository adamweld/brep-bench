//! The box from the origin to (10, 8, 6) shelled inward to a wall 3.99 thick, open at its top: a slot
//! 0.02 wide.
use springrep::edit::Wall;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // a block's faces are named by the profile's edges (`u_min` x = 0, `u_max` x = 10, `v_min`
    // y = 0, `v_max` y = 8) and its ends (`start` z = 0, `end` z = 6)
    let b = block(&at([0., 0., 0.]), Vec3::new(10., 8., 6.), "box").map_err(e)?;
    // the walls at y = 0 and y = 8 come within 0.02 of each other
    shelled(&b, &["box.end"], 3.99, Wall::Inward, "shell")
}

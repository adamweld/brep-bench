//! The box from the origin to (10, 8, 6) hollowed to a wall 1 thick with no opening: a closed cavity,
//! the box from (1, 1, 1) to (9, 7, 5).
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
    // no face open: the cavity is closed, a void inside the box
    shelled(&b, &[], 1., Wall::Inward, "shell")
}

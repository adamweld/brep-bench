//! The box from the origin to (20, 14, 10) with its four vertical edges rounded to radius 2, shelled
//! inward to a wall 1 thick, open at its top: each round moved in is the cylinder of radius 1.
use springrep::blend::fillet;
use springrep::edit::Wall;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(20., 14., 10.), "plate").map_err(e)?;
    // the four vertical edges, where the sides x = const meet the sides y = const
    let corners = [
        ("plate.u_min", "plate.v_min"),
        ("plate.v_min", "plate.u_max"),
        ("plate.u_max", "plate.v_max"),
        ("plate.v_max", "plate.u_min"),
    ];
    let rounded = fillet(&b, &corners, 2., "round").map_err(e)?;
    shelled(&rounded, &["plate.end"], 1., Wall::Inward, "shell")
}

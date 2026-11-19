//! The box from the origin to (40, 30, 10), the edge of its top and its back rounded to radius 3;
//! the top then moved up 6 and the round carried with it, unstretched.
use springrep::blend::fillet;
use springrep::edit::move_faces_by;
use springrep::math::{Transform, Vec3};
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::edit::faces;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(40., 30., 10.), "block").map_err(e)?;
    let rounded = fillet(&b, &[("block.end", "block.v_max")], 3., "round").map_err(e)?;
    // the top and its round move together, rigidly; the back and the ends run on up to them
    let moved = faces(&rounded, &["block.end", "round.round_0"])?;
    move_faces_by(&rounded, &moved, &Transform::translation(Vec3::new(0., 0., 6.)))
        .map_err(|err| format!("springrep's move_faces_by declines: {}", e(err)))
}

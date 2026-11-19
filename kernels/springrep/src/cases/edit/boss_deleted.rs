//! A disk plate of radius 8 with a boss of radius 3 standing 8 on it; the boss's side and top then
//! deleted, the plate's top closing over its foot.
//!
//! Refused: springrep's `edit::delete_faces` fills gaps, it does not take material away.
use springrep::boolean::union;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = cylinder(&at([0., 0., -4.]), 8., 4., "plate").map_err(e)?;
    let boss = cylinder(&at([0., 0., 0.]), 3., 8., "boss").map_err(e)?;
    let _part = union(&plate, &boss).map_err(e)?;
    // refused: springrep deletes faces only where the gap they leave is filled
    Err("the boss stands out of the plate: deleting its faces must take its material away, and springrep's \
         delete_faces only closes a gap by joining material to the solid (its faces must be a recess, \
         a hole or a corner cut away), so it would leave the boss in place"
        .into())
}

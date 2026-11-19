//! A disk plate of radius 8 with a boss of radius 3 on it, the concave edge at the boss's foot
//! rounded to radius 1; the round then deleted, the foot's edge sharp again.
//!
//! Refused: springrep's `edit::delete_faces` fills gaps, it does not take material away.
use springrep::blend::fillet;
use springrep::boolean::union;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = cylinder(&at([0., 0., -4.]), 8., 4., "plate").map_err(e)?;
    let boss = cylinder(&at([0., 0., 0.]), 3., 8., "boss").map_err(e)?;
    let part = union(&plate, &boss).map_err(e)?;
    let _rounded = fillet(&part, &[("plate.end", "boss.side")], 1., "round").map_err(e)?;
    // refused: springrep deletes faces only where the gap they leave is filled
    Err("the round at the boss's foot is concave, material the fillet added: deleting it must take that \
         material away, and springrep's delete_faces only closes a gap by joining material to the \
         solid (its faces must be a recess, a hole or a corner cut away), so it would leave the round \
         in place"
        .into())
}

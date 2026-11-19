//! A shaft of radius 5 from z = 0 to z = 20 with two opposite flats milled along it at x = ±4; both
//! flats then deleted together, the cylinder closing over them.
use springrep::boolean::difference_all;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::edit::{delete_named, shaft};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let east = block(&at([4., -6., -1.]), Vec3::new(3., 12., 22.), "east").map_err(e)?;
    let west = block(&at([-7., -6., -1.]), Vec3::new(3., 12., 22.), "west").map_err(e)?;
    let milled = difference_all(&shaft()?, &[&east, &west]).map_err(e)?;
    // the flats are the cutters' sides x = 4 and x = −4
    delete_named(&milled, &["east.u_min", "west.u_max"])
}

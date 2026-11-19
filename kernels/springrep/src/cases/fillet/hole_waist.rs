//! A plate 20 square and 2 thick with a hole of radius 3 through it, both rims of the hole rounded
//! to radius 1, half the plate's thickness, so the hole's wall vanishes into the inner half of a
//! torus.
use springrep::blend::fillet;
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = block(&at([-10., -10., 0.]), Vec3::new(20., 20., 2.), "plate").map_err(e)?;
    let hole = cylinder(&at([0., 0., -1.]), 3., 4., "hole").map_err(e)?;
    let body = difference(&plate, &hole).map_err(e)?;
    let pairs = [("plate.end", "hole.side"), ("plate.start", "hole.side")];
    fillet(&body, &pairs, 1., "round").map_err(e)
}

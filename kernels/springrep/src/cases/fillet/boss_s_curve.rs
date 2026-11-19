//! A square plate with a round boss of radius 5 and height 4 on it, the boss's rim rounded to
//! radius 2 (convex) and its root rounded to radius 2 (concave), so the boss's side vanishes and
//! its profile is an S.
use springrep::blend::fillet;
use springrep::boolean::union;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = block(&at([-20., -20., 0.]), Vec3::new(40., 40., 4.), "plate").map_err(e)?;
    let boss = cylinder(&at([0., 0., 4.]), 5., 4., "boss").map_err(e)?;
    let body = union(&plate, &boss).map_err(e)?;
    let pairs = [("boss.end", "boss.side"), ("plate.end", "boss.side")];
    fillet(&body, &pairs, 2., "round").map_err(e)
}

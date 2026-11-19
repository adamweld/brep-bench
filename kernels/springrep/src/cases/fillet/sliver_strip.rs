//! A plate from the origin to (20, 10, 2), the two edges of its face y = 10 that run along x
//! rounded to radius 0.9999, leaving a strip of that face only 0.0002 tall between the rounds.
use springrep::blend::fillet;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let body = block(&at([0., 0., 0.]), Vec3::new(20., 10., 2.), "plate").map_err(e)?;
    // the face y = 10 is `v_max`; it meets the bottom (`start`) and the top (`end`)
    let pairs = [("plate.v_max", "plate.start"), ("plate.v_max", "plate.end")];
    fillet(&body, &pairs, 0.9999, "round").map_err(e)
}

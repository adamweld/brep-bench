//! A block from the origin to (10, 4, 6), the two edges of its top that run along x chamfered 2 on
//! both faces, half the top's width, so the top vanishes into a ridge along y = 2, z = 6.
use springrep::blend::chamfer;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let body = block(&at([0., 0., 0.]), Vec3::new(10., 4., 6.), "block").map_err(e)?;
    // the top (`end`) meets the side y = 0 (`v_min`) and the side y = 4 (`v_max`)
    let pairs = [("block.end", "block.v_min"), ("block.end", "block.v_max")];
    chamfer(&body, &pairs, 2., "chamfer").map_err(e)
}

//! A block from the origin to (10, 4, 6), the two edges of its top that run along x rounded to
//! radius 2, half the top's width, so the top vanishes and the block's top is a half cylinder.
use springrep::blend::fillet;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let body = block(&at([0., 0., 0.]), Vec3::new(10., 4., 6.), "block").map_err(e)?;
    let pairs = [("block.end", "block.v_min"), ("block.end", "block.v_max")];
    fillet(&body, &pairs, 2., "round").map_err(e)
}

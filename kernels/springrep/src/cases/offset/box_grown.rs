//! The box from the origin to (10, 8, 6), grown by 1: every point within 1 of it, its edges
//! quarter cylinders and its corners eighths of a ball.
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(10., 8., 6.), "box").map_err(e)?;
    grown(&b, 1.)
}

//! A rectangular block 10 long in x, 20 in y and 30 in z, from the origin to (10, 20, 30).
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // a block is placed by the frame at its corner, its sizes along the frame's axes
    block(&at([0., 0., 0.]), Vec3::new(10., 20., 30.), "box").map_err(e)
}

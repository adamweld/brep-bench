//! The box from the origin to (20, 10, 10), the two long edges of its top rounded to radius 2; both
//! rounds then deleted together, giving the box back.
use springrep::blend::fillet;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(20., 10., 10.), "block").map_err(e)?;
    let edges = [("block.end", "block.v_min"), ("block.end", "block.v_max")];
    let rounded = fillet(&b, &edges, 2., "round").map_err(e)?;
    delete_feature(&rounded, "round")
}

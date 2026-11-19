//! The box from the origin to (20, 16, 12), every edge rounded to radius 2; all twenty rounds then
//! deleted, the six flat faces running on to meet in the box's edges and corners.
use springrep::blend::fillet_edges;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(20., 16., 12.), "box").map_err(e)?;
    let every: Vec<_> = b.edge_ids().collect();
    let rounded = fillet_edges(&b, &every, 2., "round").map_err(e)?;
    // the twelve rounds and the eight corner patches are all the fillet's
    delete_feature(&rounded, "round")
}

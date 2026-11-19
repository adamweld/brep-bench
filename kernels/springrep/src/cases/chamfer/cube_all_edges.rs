//! A cube of side 10 with all twelve edges chamfered 2, in one operation: three chamfers meet at
//! each corner, which is cut flat by a triangle.
use springrep::blend::chamfer_edges;
use springrep::math::Vec3;
use springrep::pick::convex_edges;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let cube = block(&at([0., 0., 0.]), Vec3::new(10., 10., 10.), "cube").map_err(e)?;
    let edges = convex_edges(&cube).map_err(e)?;
    chamfer_edges(&cube, &edges, 2., "chamfer").map_err(e)
}

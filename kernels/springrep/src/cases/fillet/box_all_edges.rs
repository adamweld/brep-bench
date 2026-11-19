//! A cube of side 10 with all twelve edges rounded to radius 1, in one operation: three rounds
//! meet at each corner, closed by a ball.
use springrep::blend::fillet_edges;
use springrep::math::Vec3;
use springrep::pick::convex_edges;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let cube = block(&at([0., 0., 0.]), Vec3::new(10., 10., 10.), "cube").map_err(e)?;
    let edges = convex_edges(&cube).map_err(e)?;
    fillet_edges(&cube, &edges, 1., "round").map_err(e)
}

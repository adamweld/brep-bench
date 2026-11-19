//! A block from the origin to (20, 10, 10), the edge where its top meets its end x = 20 blended
//! by a conic of rho 0.5, a parabola, tangent to both faces 3 from the edge.
use springrep::blend::{EdgeName, fillet_conic};
use springrep::math::Vec3;
use springrep::pick::edges_named;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let body = block(&at([0., 0., 0.]), Vec3::new(20., 10., 10.), "block").map_err(e)?;
    let edge = EdgeName { a: "block.end", b: "block.u_max", exact: true };
    let edges = edges_named(&body, &edge).map_err(e)?;
    fillet_conic(&body, &edges, 3., 0.5, "conic").map_err(e)
}

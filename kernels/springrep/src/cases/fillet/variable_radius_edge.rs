//! A cube of side 10 from the origin, the edge where its faces x = 10 and y = 10 meet rounded with
//! a radius growing evenly from 1 at the bottom to 2 at the top.
use springrep::blend::{EdgeName, fillet_variable};
use springrep::math::Vec3;
use springrep::pick::edges_named;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let cube = block(&at([0., 0., 0.]), Vec3::new(10., 10., 10.), "cube").map_err(e)?;
    let edge = EdgeName { a: "cube.u_max", b: "cube.v_max", exact: true };
    let [edge] = edges_named(&cube, &edge).map_err(e)?[..] else {
        return Err("the two sides meet in more than one edge".into());
    };
    // radius 1 at the end nearest the bottom, 2 at the top
    fillet_variable(&cube, edge, Vec3::new(10., 10., 0.), 1., 2., "round").map_err(e)
}

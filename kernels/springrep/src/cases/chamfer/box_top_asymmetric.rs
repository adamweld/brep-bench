//! A box from the origin to (20, 10, 6), the four edges round its top chamfered with two
//! distances, 1 across the top and 2 down each side.
use springrep::blend::{EdgeName, chamfer_two_named};
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let body = block(&at([0., 0., 0.]), Vec3::new(20., 10., 6.), "box").map_err(e)?;
    // each pair's first face, the top, takes the first distance
    let edges: Vec<EdgeName> = ["box.v_min", "box.u_max", "box.v_max", "box.u_min"]
        .into_iter()
        .map(|side| EdgeName { a: "box.end", b: side, exact: true })
        .collect();
    chamfer_two_named(&body, &edges, 1., 2., "chamfer").map_err(e)
}

//! The box from the origin to (20, 10, 10), the edge of its top and its side x = 20 beveled 3; the
//! bevel then deleted, the top and the side meeting again in the box's edge.
use springrep::blend::chamfer;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(20., 10., 10.), "block").map_err(e)?;
    let beveled = chamfer(&b, &[("block.end", "block.u_max")], 3., "bevel").map_err(e)?;
    delete_feature(&beveled, "bevel")
}

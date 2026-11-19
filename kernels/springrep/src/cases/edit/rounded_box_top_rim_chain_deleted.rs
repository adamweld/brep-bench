//! The box from the origin to (20, 12, 8), its upright edges rounded to radius 3 and then the whole
//! rim of its top to radius 1, a closed tangent chain of eight faces; the chain then deleted.
use springrep::blend::fillet;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(20., 12., 8.), "box").map_err(e)?;
    let uprights = [
        ("box.u_min", "box.v_min"),
        ("box.v_min", "box.u_max"),
        ("box.u_max", "box.v_max"),
        ("box.v_max", "box.u_min"),
    ];
    let rounded = fillet(&b, &uprights, 3., "corner").map_err(e)?;
    // one edge of the top's rim names the chain; the fillet runs round it tangentially
    let rimmed = fillet(&rounded, &[("box.end", "box.u_min")], 1., "rim").map_err(e)?;
    delete_feature(&rimmed, "rim")
}

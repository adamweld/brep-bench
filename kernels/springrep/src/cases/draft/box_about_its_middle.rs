//! The box from the origin to (20, 12, 10), its four sides drafted 5° about the plane z = 5,
//! halfway up: out below it, in above it.
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::draft::draft_up;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(20., 12., 10.), "box").map_err(e)?;
    draft_up(&b, &["box.u_min", "box.u_max", "box.v_min", "box.v_max"], 5., 5.)
}

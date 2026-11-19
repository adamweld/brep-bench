//! The box from the origin to (20, 12, 10), its four sides drafted 5° outward about its floor: a
//! negative draft.
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::draft::draft_up;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(20., 12., 10.), "box").map_err(e)?;
    // a negative angle leans each side outward as it rises
    draft_up(&b, &["box.u_min", "box.u_max", "box.v_min", "box.v_max"], 0., -5.)
}

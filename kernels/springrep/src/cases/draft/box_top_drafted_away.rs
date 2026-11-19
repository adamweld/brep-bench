//! The box from the origin to (30, 10, 10), its four sides drafted 30° about its floor, so far
//! that they meet in a ridge below the top: a hip roof of five faces.
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::draft::draft_up;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(30., 10., 10.), "box").map_err(e)?;
    // the draft is followed past the moment the top closes, its faces settled where they meet
    draft_up(&b, &["box.u_min", "box.u_max", "box.v_min", "box.v_max"], 0., 30.)
}

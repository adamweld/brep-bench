//! A shaft of radius 5 from z = 0 to z = 20 with a flat milled along it where y < −4; the flat then
//! deleted, the cylinder closing over it and the ends growing whole again.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::edit::{delete_named, shaft};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let mill = block(&at([-6., -6., -1.]), Vec3::new(12., 2., 22.), "mill").map_err(e)?;
    let milled = difference(&shaft()?, &mill).map_err(e)?;
    // the flat is the cutter's side y = −4
    // springrep's delete_faces fills the gap with a block sized by the deleted faces' edges alone;
    // a flat's edges all lie in its plane, so that block has no thickness and the kernel declines
    delete_named(&milled, &["mill.v_max"]).map_err(|why| {
        format!("{why} (the block that would fill the gap is sized by the flat's edges alone, which all lie in its plane)")
    })
}

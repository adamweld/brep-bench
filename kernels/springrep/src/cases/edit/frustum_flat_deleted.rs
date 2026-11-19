//! A frustum of radius 6 at z = 0 narrowing to 3 at z = 20, with a flat milled along its side where
//! y < −4; the flat then deleted, the cone closing over it and the base growing whole again.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, cone};
use springrep::topo::Brep;

use crate::shared::edit::delete_named;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let frustum = cone(&at([0., 0., 0.]), 6., 3., 20., "frustum").map_err(e)?;
    let mill = block(&at([-7., -7., -1.]), Vec3::new(14., 3., 22.), "mill").map_err(e)?;
    let milled = difference(&frustum, &mill).map_err(e)?;
    // the flat is the cutter's side y = −4
    // springrep's delete_faces fills the gap with a block sized by the deleted faces' edges alone;
    // a flat's edges all lie in its plane, so that block has no thickness and the kernel declines
    delete_named(&milled, &["mill.v_max"]).map_err(|why| {
        format!("{why} (the block that would fill the gap is sized by the flat's edges alone, which all lie in its plane)")
    })
}

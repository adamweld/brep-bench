//! A ball of radius 5 cut flat at z = 3, the flat then deleted: the ball closes over it again.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, sphere};
use springrep::topo::Brep;

use crate::shared::edit::delete_named;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)?;
    let above = block(&at([-6., -6., 3.]), Vec3::new(12., 12., 3.), "cut").map_err(e)?;
    let flattened = difference(&ball, &above).map_err(e)?;
    // the flat is the cutter's floor
    // springrep's delete_faces fills the gap with a block sized by the deleted faces' edges alone;
    // a flat's edges all lie in its plane, so that block has no thickness and the kernel declines
    delete_named(&flattened, &["cut.start"]).map_err(|why| {
        format!("{why} (the block that would fill the gap is sized by the flat's edges alone, which all lie in its plane)")
    })
}

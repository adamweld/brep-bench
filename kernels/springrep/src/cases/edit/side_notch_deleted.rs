//! The box from the origin to (20, 10, 10) with a half-round notch of radius 3 cut into its side
//! y = 0, top to bottom; the notch's wall then deleted, the side running on across it.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::edit::delete_named;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(20., 10., 10.), "block").map_err(e)?;
    let notch = cylinder(&at([10., 0., -1.]), 3., 12., "notch").map_err(e)?;
    let notched = difference(&b, &notch).map_err(e)?;
    delete_named(&notched, &["notch.side"])
}

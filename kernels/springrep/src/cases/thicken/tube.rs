//! The whole cylinder of radius 5 about the z axis from z = 0 to z = 10, a sheet, thickened by 1
//! away from the axis: the tube between radii 5 and 6.
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::thicken::thickened;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the sheet is the side of a solid cylinder; its ends, square to it, are the walls its circles
    // sweep
    let can = cylinder(&at([0., 0., 0.]), 5., 10., "can").map_err(e)?;
    thickened(&can, &["can.side"], 1.)
}

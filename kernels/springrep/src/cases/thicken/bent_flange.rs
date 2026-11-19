//! A sheet bent as sheet metal, a floor, a quarter-cylinder bend of radius 2 and an upright wall,
//! swept 8 along y, thickened by 1 to the outside of the bend: a bracket 1 thick, its bend's radii 2
//! and 3.
use springrep::topo::Brep;

use crate::shared::thicken::{flange, thickened};

pub fn build() -> Result<Brep, String> {
    // the sheet's faces face away from the bend's axis: thicken along their normals
    thickened(&flange()?, &["flange.floor", "flange.bend", "flange.wall"], 1.)
}

//! A sheet bent as sheet metal, a floor, a quarter-cylinder bend of radius 2 and an upright wall,
//! swept 8 along y, thickened by 3 to the inside of the bend, more than the bend's radius: the bend
//! moved 3 turns inside out and drops, and the floor and wall moved meet in a concave edge.
use springrep::topo::Brep;

use crate::shared::thicken::{flange, thickened};

pub fn build() -> Result<Brep, String> {
    // a negative thickness thickens into the solid, toward the bend's axis
    thickened(&flange()?, &["flange.floor", "flange.bend", "flange.wall"], -3.)
}

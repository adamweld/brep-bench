//! A narrow cup of revolution, a floor of radius 3, a bend of radius 2 and a wall of radius 5 up to
//! z = 10, a sheet, thickened by 4 to the inside, twice the bend's radius: the bend moved turns inside
//! out and drops, and the floor and wall moved meet in the circle of radius 1 at z = 4.
use springrep::topo::Brep;

use crate::shared::thicken::{cup, thickened};

pub fn build() -> Result<Brep, String> {
    // a negative thickness thickens into the solid: up from the floor and in from the wall
    thickened(&cup(3., 2., 10.)?, &["cup.floor", "cup.bend", "cup.wall"], -4.)
}

//! A pan of revolution, a floor of radius 5, a bend of radius 1 and a wall of radius 6 up to z = 8, a
//! sheet, thickened by 3 to the inside, three times the bend's radius: the bend moved turns inside
//! out and drops, and the floor and wall moved meet in the circle of radius 3 at z = 3.
use springrep::topo::Brep;

use crate::shared::thicken::{cup, thickened};

pub fn build() -> Result<Brep, String> {
    // a negative thickness thickens into the solid: up from the floor and in from the wall
    thickened(&cup(5., 1., 8.)?, &["cup.floor", "cup.bend", "cup.wall"], -3.)
}

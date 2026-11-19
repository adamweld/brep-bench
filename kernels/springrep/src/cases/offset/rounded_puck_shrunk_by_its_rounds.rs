//! The cylinder of radius 6 on the z axis from z = 0 to z = 10, both rims rounded to radius 2,
//! shrunk by 2, exactly the rounds' radius: each round shrinks onto its tube's centre circle,
//! leaving the cylinder of radius 4 from z = 2 to z = 8.
//! springrep 1.0.1 refuses it: its face offset cannot follow a round moved onto its centre circle
//! ("an offset … closes a torus to nothing").
use springrep::blend::fillet;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::offset::shrunk;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let puck = cylinder(&at([0., 0., 0.]), 6., 10., "puck").map_err(e)?;
    let rims = [("puck.start", "puck.side"), ("puck.end", "puck.side")];
    let rounded = fillet(&puck, &rims, 2., "round").map_err(e)?;
    shrunk(&rounded, 2.)
}

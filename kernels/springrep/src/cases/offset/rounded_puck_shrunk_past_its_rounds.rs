//! The cylinder of radius 6 on the z axis from z = 0 to z = 10, both rims rounded to radius 2,
//! shrunk by 3, past the rounds' radius: the rounds vanish, leaving the cylinder of radius 3 from
//! z = 3 to z = 7.
//! springrep 1.0.1 refuses it: its face offset cannot follow a round moved past its radius ("an
//! offset … closes a torus to nothing").
use springrep::blend::fillet;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::offset::shrunk;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let puck = cylinder(&at([0., 0., 0.]), 6., 10., "puck").map_err(e)?;
    let rims = [("puck.start", "puck.side"), ("puck.end", "puck.side")];
    let rounded = fillet(&puck, &rims, 2., "round").map_err(e)?;
    shrunk(&rounded, 3.)
}

//! The cylinder of radius 8 on the z axis from z = 0 to z = 6 with a flange round its side, from
//! radius 8 out to 11 and z = 2.2 to 3.8, shrunk by 1: the flange is gone but for a low ridge.
//! springrep 1.0.1 refuses it: the offset of the complement does not follow the flange's faces
//! through each other ("a moved edge runs backwards").
use springrep::boolean::union;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::offset::shrunk;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let drum = cylinder(&at([0., 0., 0.]), 8., 6., "drum").map_err(e)?;
    let flange = cylinder(&at([0., 0., 2.2]), 11., 1.6, "flange").map_err(e)?;
    let flanged = union(&drum, &flange).map_err(e)?;
    shrunk(&flanged, 1.)
}

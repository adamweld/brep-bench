//! The cylinder of radius 10 on the z axis from z = 0 to z = 10, with a groove 1.6 wide (z = 4.2 to
//! 5.8) cut 3 deep round its side, grown by 1: the groove closes, leaving a shallow V notch.
//! springrep 1.0.1 refuses it: its offset does not follow the groove's walls through each other ("a
//! moved edge runs backwards").
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let drum = cylinder(&at([0., 0., 0.]), 10., 10., "drum").map_err(e)?;
    // the groove: a ring from radius 7 out past the side
    let outer = cylinder(&at([0., 0., 4.2]), 11., 1.6, "groove").map_err(e)?;
    let core = cylinder(&at([0., 0., 4.]), 7., 2., "core").map_err(e)?;
    let groove = difference(&outer, &core).map_err(e)?;
    let grooved = difference(&drum, &groove).map_err(e)?;
    grown(&grooved, 1.)
}

//! A shaft of radius 5 from z = 0 to z = 20 with a ring groove down to radius 4 from z = 8 to
//! z = 12; the groove's floor and walls then deleted, the cylinder running on across it.
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::edit::{delete_named, shaft};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the turning tool: the ring outside radius 4 from z = 8 to z = 12
    let ring = cylinder(&at([0., 0., 8.]), 6., 4., "ring").map_err(e)?;
    let core = cylinder(&at([0., 0., 7.]), 4., 6., "core").map_err(e)?;
    let tool = difference(&ring, &core).map_err(e)?;
    let grooved = difference(&shaft()?, &tool).map_err(e)?;
    delete_named(&grooved, &["core.side", "ring.start", "ring.end"])
}

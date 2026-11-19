//! A frustum of radius 6 at z = 0 narrowing to 3 at z = 20, with a ring groove down to radius 4
//! from z = 8 to z = 12; the groove's three faces then deleted, the cone running on across it.
use springrep::boolean::difference;
use springrep::primitive::{cone, cylinder};
use springrep::topo::Brep;

use crate::shared::edit::delete_named;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let frustum = cone(&at([0., 0., 0.]), 6., 3., 20., "frustum").map_err(e)?;
    // the turning tool: the ring outside radius 4 from z = 8 to z = 12
    let ring = cylinder(&at([0., 0., 8.]), 10., 4., "ring").map_err(e)?;
    let core = cylinder(&at([0., 0., 7.]), 4., 6., "core").map_err(e)?;
    let tool = difference(&ring, &core).map_err(e)?;
    let grooved = difference(&frustum, &tool).map_err(e)?;
    delete_named(&grooved, &["core.side", "ring.start", "ring.end"])
}

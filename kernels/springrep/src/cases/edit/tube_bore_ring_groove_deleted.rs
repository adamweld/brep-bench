//! A tube of outer radius 10 and bore 4 from z = 0 to z = 20 with a ring groove out to radius 6
//! from z = 8 to z = 12 turned into its bore; the groove's three faces then deleted.
use springrep::boolean::difference_all;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::edit::delete_named;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let tube = cylinder(&at([0., 0., 0.]), 10., 20., "tube").map_err(e)?;
    let bore = cylinder(&at([0., 0., -1.]), 4., 22., "bore").map_err(e)?;
    let groove = cylinder(&at([0., 0., 8.]), 6., 4., "groove").map_err(e)?;
    let part = difference_all(&tube, &[&bore, &groove]).map_err(e)?;
    // its floor, the cylinder of radius 6, and its walls at z = 8 and z = 12
    delete_named(&part, &["groove.side", "groove.start", "groove.end"]).map_err(|why| {
        format!("{why} (the bore's wall, hollow: springrep closes a gap only against planes and cylinders bulging outward)")
    })
}

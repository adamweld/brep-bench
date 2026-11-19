//! The cone on the z axis with a base of radius 3 at z = 0 and its apex at (0, 0, 4), grown by 1.
//! springrep 1.0.1 builds a face off its own edges round the apex (0.31 mm), which its checker
//! refuses.
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let c = cone(&at([0., 0., 0.]), 3., 0., 4., "cone").map_err(e)?;
    grown(&c, 1.)
}

//! The ball of radius 15 about the origin united with the cone whose apex is (0, 0, -25) and whose
//! base is the disk of radius 12 at z = -9: its side is tangent to the sphere along the base's rim.
use springrep::boolean::union;
use springrep::primitive::{cone, sphere};
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let scoop = sphere(&at([0., 0., 0.]), 15., "scoop").map_err(e)?;
    let wafer = cone(&at([0., 0., -25.]), 0., 12., 16., "cone").map_err(e)?;
    union(&scoop, &wafer).map_err(e)
}

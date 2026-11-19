//! The cylinder of radius 5 on the z axis from z = 0 to z = 10, grown by 1.
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let c = cylinder(&at([0., 0., 0.]), 5., 10., "drum").map_err(e)?;
    grown(&c, 1.)
}

//! The block under the bicubic Bézier dome, drilled straight down by a hole of radius 3 about the
//! line x = 12, y = 8.
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::nurbs::bezier_dome;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let dome = bezier_dome()?;
    let drill = cylinder(&at([12., 8., -1.]), 3., 30., "drill").map_err(e)?;
    difference(&dome, &drill).map_err(e)
}

//! The cylinder of radius 0.0007575 and height 0.0015 about the z axis from z = 0, less the
//! cylinder of radius 0.00075 between the same planes: a tube whose wall is 7.5·10⁻⁶ thick.
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let wide = cylinder(&at([0., 0., 0.]), 0.0007575, 0.0015, "wide").map_err(e)?;
    let small = cylinder(&at([0., 0., 0.]), 0.00075, 0.0015, "cylinder").map_err(e)?;
    difference(&wide, &small).map_err(e)
}

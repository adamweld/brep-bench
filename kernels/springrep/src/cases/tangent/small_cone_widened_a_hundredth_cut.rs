//! The cone of base radius 0.0007575 at z = 0 and apex (0, 0, 0.0015), less the cone of base
//! radius 0.00075 with the same base plane and apex: a conical shell 7.5·10⁻⁶ thick at its base.
use springrep::boolean::difference;
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let wide = cone(&at([0., 0., 0.]), 0.0007575, 0., 0.0015, "wide").map_err(e)?;
    let small = cone(&at([0., 0., 0.]), 0.00075, 0., 0.0015, "cone").map_err(e)?;
    difference(&wide, &small).map_err(e)
}

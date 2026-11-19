//! The cone of base radius 5 at z = 0 and apex at (0, 0, 10), less the cone of base radius 4.999
//! with the same base plane and apex: a conical shell 0.001 thick at its base.
use springrep::boolean::difference;
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let outer = cone(&at([0., 0., 0.]), 5., 0., 10., "outer").map_err(e)?;
    let inner = cone(&at([0., 0., 0.]), 4.999, 0., 10., "inner").map_err(e)?;
    difference(&outer, &inner).map_err(e)
}

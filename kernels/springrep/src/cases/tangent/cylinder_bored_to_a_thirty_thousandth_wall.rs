//! The cylinder of radius 5 from z = 0 to 10 less the coaxial cylinder of radius 4.99997 between the
//! same two planes: a tube whose wall is 0.00003 thick.
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let outer = cylinder(&at([0., 0., 0.]), 5., 10., "outer").map_err(e)?;
    let inner = cylinder(&at([0., 0., 0.]), 4.99997, 10., "inner").map_err(e)?;
    difference(&outer, &inner).map_err(e)
}

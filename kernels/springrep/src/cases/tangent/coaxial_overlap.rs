//! Two cylinders of radius 5 on the z axis, from z = 0 to 10 and from 6 to 16, united into one: the
//! walls coincide between 6 and 10, the second turned a quarter turn so their seams differ.
use springrep::boolean::union;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    let lower = cylinder(&at([0., 0., 0.]), 5., 10., "lower").map_err(e)?;
    // the second's seam toward +y, a quarter turn from the first's toward +x
    let upper = cylinder(&frame([0., 0., 6.], [0., 0., 1.], [0., 1., 0.])?, 5., 10., "upper")
        .map_err(e)?;
    union(&lower, &upper).map_err(e)
}

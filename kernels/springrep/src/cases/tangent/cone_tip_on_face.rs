//! The block [-10, 10]²×[0, 10] united with the cone whose apex is the middle of its bottom face
//! and whose base is the disk of radius 5 at z = 15: the tip touches the face from inside.
use springrep::boolean::union;
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::tangent::between;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let slab = between([-10., -10., 0.], [10., 10., 10.], "block")?;
    // apex (radius 0) at the origin, radius 5 at the top
    let tip = cone(&at([0., 0., 0.]), 0., 5., 15., "cone").map_err(e)?;
    union(&slab, &tip).map_err(e)
}

//! The block [-10, 10]²×[0, 10] united with the solid paraboloid x² + y² ≤ 8z, z ≤ 12.5, whose
//! vertex touches the middle of the block's bottom face from inside.
use springrep::boolean::union;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::tangent::{between, paraboloid};

pub fn build() -> Result<Brep, String> {
    let slab = between([-10., -10., 0.], [10., 10., 10.], "block")?;
    // x² + y² = 8z: a focal length of 2
    let solid = paraboloid(2., 12.5, "paraboloid")?;
    union(&slab, &solid).map_err(e)
}

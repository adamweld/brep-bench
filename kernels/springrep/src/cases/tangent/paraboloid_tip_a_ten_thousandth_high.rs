//! The tip of the solid paraboloid x² + y² ≤ 8z, z ≤ 8: its part below the plane z = 10⁻⁴, a disk
//! 0.057 across over a bowl 10⁻⁴ deep.
use springrep::boolean::intersection;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::tangent::{between, paraboloid};

pub fn build() -> Result<Brep, String> {
    // x² + y² = 8z: a focal length of 2
    let solid = paraboloid(2., 8., "paraboloid")?;
    // the half-space below z = 10⁻⁴, past the paraboloid on every side but its top
    let below = between([-10., -10., -1.], [10., 10., 1e-4], "below")?;
    intersection(&solid, &below).map_err(e)
}

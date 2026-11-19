//! The cube [-5, 5]³ united with the cylinder of radius 5 on the x axis from x = -10 to 10, which
//! is inscribed in it, touching the faces y = ±5 and z = ±5 along their middle lines.
use springrep::boolean::union;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::tangent::{between, rod};

pub fn build() -> Result<Brep, String> {
    let cube = between([-5., -5., -5.], [5., 5., 5.], "cube")?;
    // the seam along the line y = z of the wall, at 45° to the lines of contact
    let shaft = rod([-10., 0., 0.], [1., 0., 0.], [0., 1., 1.], 5., 20., "shaft")?;
    union(&cube, &shaft).map_err(e)
}

//! Two tori of radii 3 and 0.5, one about z centred on the origin and one about a line along y
//! through (3, 0, 0), linked through each other without touching, united: two separate rings.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::torus;
use cad_kernel::topo::Brep;

const MAJOR: f64 = 3.;
const MINOR: f64 = 0.5;

pub fn build() -> Result<Brep, String> {
    let first = torus([0., 0., 0.], [0., 0., 1.], MAJOR, MINOR)?;
    // its middle circle in the plane y = 0, through the origin, inside the first ring's disk, and
    // through (6, 0, 0), outside it
    let second = torus([MAJOR, 0., 0.], [0., 1., 0.], MAJOR, MINOR)?;
    // the union keeps both bodies, though they share no point
    Ok(combined(&first, &second, Op::Union, 0.)?)
}

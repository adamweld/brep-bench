//! Four balls of radius 10 centred on the corners of a regular tetrahedron of side 10, (0, 0, 0),
//! (10, 0, 0), (5, 5√3, 0) and (5, 5√3/3, 10√(2/3)): the solid common to all four, a Reuleaux
//! tetrahedron.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::topo::Brep;

/// The tetrahedron's side, and each ball's radius.
const SIDE: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    let (s, r3) = (SIDE, 3f64.sqrt());
    let corners = [[0., 0., 0.], [s, 0., 0.], [s / 2., s * r3 / 2., 0.],
                   [s / 2., s * r3 / 6., s * (2f64 / 3.).sqrt()]];
    // each ball taken in common with what the ones before it left
    let mut body = sphere(corners[0], [0., 0., 1.], SIDE)?;
    for &c in &corners[1..] {
        body = combined(&body, &sphere(c, [0., 0., 1.], SIDE)?, Op::Common, 0.)?;
    }
    Ok(body)
}

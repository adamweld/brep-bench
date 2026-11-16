//! A small cube, of side 0.001, from the origin to (0.001, 0.001, 0.001), and a copy of itself
//! turned 1.2° about the z axis, the line of one of the cube's own edges: the part common to both,
//! a prism over the quadrilateral the two squares share.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::geom::Rigid;
use cad_kernel::topo::Brep;

const SIDE: f64 = 0.001;
const TURN_DEGREES: f64 = 1.2;

pub fn build() -> Result<Brep, String> {
    let cube = block([0.; 3], [SIDE; 3])?;
    let turned = cube.moved(&Rigid::turn([0., 0., 0.], [0., 0., 1.], TURN_DEGREES.to_radians()));
    // the two share the edge on the z axis and their top and bottom planes
    Ok(combined(&cube, &turned, Op::Common, 0.)?)
}

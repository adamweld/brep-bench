//! A small cube, of side 0.0015, from the origin to (0.0015, 0.0015, 0.0015), less a copy of itself
//! raised by 0.000015 along z, a hundredth of its side: the slab under the copy is left, a
//! hundredth as thick as it is wide, its top the copy's bottom.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::geom::Rigid;
use cad_kernel::topo::Brep;

const SIDE: f64 = 0.0015;
const RISE: f64 = SIDE / 100.;

pub fn build() -> Result<Brep, String> {
    let cube = block([0.; 3], [SIDE; 3])?;
    let raised = cube.moved(&Rigid { t: [0., 0., RISE], ..Rigid::identity() });
    // the floor of 0 lets the kernel's tolerance follow the model's size
    Ok(combined(&cube, &raised, Op::Cut, 0.)?)
}

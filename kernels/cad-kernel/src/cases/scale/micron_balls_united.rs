//! Two balls united: one of radius 0.005 about the origin, one of radius 0.004 about
//! (0.002, 0.003, 0.006), 0.007 from the first.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // each ball's poles on the vertical through its centre
    let first = sphere([0., 0., 0.], [0., 0., 1.], 0.005)?;
    let second = sphere([0.002, 0.003, 0.006], [0., 0., 1.], 0.004)?;
    Ok(combined(&first, &second, Op::Union, 0.)?)
}

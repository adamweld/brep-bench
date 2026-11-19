//! Two balls united: one of radius 50000 about the origin, one of radius 40000 about
//! (20000, 30000, 60000), 70000 from the first.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // each ball's poles on the vertical through its centre
    let first = sphere([0., 0., 0.], [0., 0., 1.], 50000.)?;
    let second = sphere([20000., 30000., 60000.], [0., 0., 1.], 40000.)?;
    Ok(combined(&first, &second, Op::Union, 0.)?)
}

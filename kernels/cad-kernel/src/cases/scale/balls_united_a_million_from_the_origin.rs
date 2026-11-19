//! Two balls a million from the origin, united: one of radius 5 about (600000, −800000, 0), one of
//! radius 4 about (600002, −799997, 6), 7 from the first along (2, 3, 6)/7.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::topo::Brep;

const CENTRE: [f64; 3] = [600000., -800000., 0.];

pub fn build() -> Result<Brep, String> {
    let [x, y, z] = CENTRE;
    // each ball's poles on the vertical through its centre
    let first = sphere(CENTRE, [0., 0., 1.], 5.)?;
    let second = sphere([x + 2., y + 3., z + 6.], [0., 0., 1.], 4.)?;
    Ok(combined(&first, &second, Op::Union, 0.)?)
}

//! Balls of radius 5 about the origin, 4 about (7, 0, 0) and 3 about (10, 4, 0), united one after
//! another, with everything below the plane z = 0 through their centres cut away.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{kept, sphere};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // each ball's poles on the vertical through its centre
    let first = sphere([0., 0., 0.], [0., 0., 1.], 5.)?;
    let second = sphere([7., 0., 0.], [0., 0., 1.], 4.)?;
    let third = sphere([10., 4., 0.], [0., 0., 1.], 3.)?;
    // the first two, then the third with that
    let two = combined(&first, &second, Op::Union, 0.)?;
    let chain = combined(&two, &third, Op::Union, 0.)?;
    // the part above the xy plane
    kept(&chain, [0., 0., 0.], [0., 0., 1.], 0.)
}

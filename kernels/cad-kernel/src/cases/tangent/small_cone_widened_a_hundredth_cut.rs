//! A small cone, its base of radius 0.00075 at z = 0 and its apex 0.0015 above it on the z axis,
//! and a copy whose base is a hundredth wider, radius 0.0007575, with the same base plane and apex:
//! the copy less the cone, a conical shell 0.0000075 thick at its base, thinning to nothing at the
//! apex the two share.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::cone;
use cad_kernel::topo::Brep;

const RADIUS: f64 = 0.00075;
const WIDER: f64 = RADIUS * 1.01;
const HEIGHT: f64 = 0.0015;

pub fn build() -> Result<Brep, String> {
    // each from its base up to its apex, a radius of nothing there
    let inner = cone([0., 0., 0.], [0., 0., 1.], RADIUS, 0., HEIGHT)?;
    let outer = cone([0., 0., 0.], [0., 0., 1.], WIDER, 0., HEIGHT)?;
    Ok(combined(&outer, &inner, Op::Cut, 0.)?)
}

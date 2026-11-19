//! Two cylinders of radius 3 and length 20 crossing square at c = (600000, −800000, 0), one along
//! (2, 3, 6)/7 and one along (3, −6, 2)/7, each from c − 10u to c + 10u: their common part.
use crate::solids::*;

/// The cylinder of radius 3 along `u` (a unit vector), 10 either way from `c`.
fn through(c: Vec3, u: Vec3) -> R<Body> {
    solidcraft_kernel::cylinder(c - u * 10.0, u, 3.0, 20.0)
}

pub fn build() -> R<Body> {
    let c = v3(600000.0, -800000.0, 0.0);
    let a = through(c, v3(2.0, 3.0, 6.0) * (1.0 / 7.0))?;
    let b = through(c, v3(3.0, -6.0, 2.0) * (1.0 / 7.0))?;
    common(&a, &b)
}

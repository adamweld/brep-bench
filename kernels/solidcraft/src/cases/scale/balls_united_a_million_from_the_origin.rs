//! A ball of radius 5 about c = (600000, −800000, 0) united with a ball of radius 4 about
//! c + (2, 3, 6), 7 from the first along a direction no axis lines up with.
use crate::solids::*;

pub fn build() -> R<Body> {
    let c = v3(600000.0, -800000.0, 0.0);
    let a = solidcraft_kernel::sphere(c, 5.0)?;
    let b = solidcraft_kernel::sphere(c + v3(2.0, 3.0, 6.0), 4.0)?;
    union(&a, &b)
}

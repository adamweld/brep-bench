use crate::solids::*;

/// Balls of radius 5, 4 and 3 about the origin, (7, 0, 0) and (10, 4, 0), united one after
/// another: the first two, then the third with that.
pub fn chain() -> R<Body> {
    let a = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    let b = solidcraft_kernel::sphere(v3(7.0, 0.0, 0.0), 4.0)?;
    let c = solidcraft_kernel::sphere(v3(10.0, 4.0, 0.0), 3.0)?;
    union(&union(&a, &b)?, &c)
}

pub fn build() -> R<Body> {
    chain()
}

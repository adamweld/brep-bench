use crate::solids::*;

pub fn build() -> R<Body> {
    let a = solidcraft_kernel::sphere(Vec3::ZERO, 50000.0)?;
    let b = solidcraft_kernel::sphere(v3(20000.0, 30000.0, 60000.0), 40000.0)?;
    union(&a, &b)
}

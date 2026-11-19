use crate::solids::*;

pub fn build() -> R<Body> {
    let a = solidcraft_kernel::sphere(Vec3::ZERO, 5.511)?;
    let b = solidcraft_kernel::sphere(v3(4.231, 3.467, 2.653), 3.317)?;
    common(&a, &b)
}

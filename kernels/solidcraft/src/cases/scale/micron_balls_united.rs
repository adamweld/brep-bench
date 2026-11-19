use crate::solids::*;

pub fn build() -> R<Body> {
    let a = solidcraft_kernel::sphere(Vec3::ZERO, 0.005)?;
    let b = solidcraft_kernel::sphere(v3(0.002, 0.003, 0.006), 0.004)?;
    union(&a, &b)
}

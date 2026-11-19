use crate::solids::*;

pub fn build() -> R<Body> {
    let a = solidcraft_kernel::cylinder(v3(-0.01, 0.0, 0.0), Vec3::X, 0.003, 0.02)?;
    let b = solidcraft_kernel::cylinder(v3(0.0, -0.01, 0.0), Vec3::Y, 0.003, 0.02)?;
    common(&a, &b)
}

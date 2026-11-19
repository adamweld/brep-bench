use crate::solids::*;

/// The half of `torus` on the positive side of the plane through the origin with normal `n`.
fn half(torus: &Body, n: Vec3) -> R<Body> {
    let plane = Plane::from_normal(Vec3::ZERO, n).ok_or_else(|| KernelError::Invalid("plane".into()))?;
    solidcraft_kernel::half_space(torus, &plane, true)?
        .0
        .ok_or_else(|| KernelError::Failed("the half torus is empty".into()))
}

pub fn build() -> R<Body> {
    let first = solidcraft_kernel::torus(Vec3::ZERO, 10.0, 3.0)?;
    // the second turned a quarter turn about z, so the faces that coincide are not split in the
    // same places
    let second = solidcraft_kernel::transform(&first, Vec3::ZERO, Vec3::ZERO, Vec3::Z, std::f64::consts::FRAC_PI_2)?;
    union(&half(&first, Vec3::Z)?, &half(&second, Vec3::X)?)
}

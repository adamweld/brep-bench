use crate::solids::*;

/// The half of `ball` on the positive side of the plane through the origin with normal `n`.
fn half(ball: &Body, n: Vec3) -> R<Body> {
    let plane = Plane::from_normal(Vec3::ZERO, n).ok_or_else(|| KernelError::Invalid("plane".into()))?;
    solidcraft_kernel::half_space(ball, &plane, true)?
        .0
        .ok_or_else(|| KernelError::Failed("the half ball is empty".into()))
}

pub fn build() -> R<Body> {
    // the kernel's ball has its poles on the z axis; the second ball is turned a quarter turn
    // about y, putting its poles on the x axis, so each half's flat face holds the other's poles
    let first = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    let second = solidcraft_kernel::transform(&first, Vec3::ZERO, Vec3::ZERO, Vec3::Y, std::f64::consts::FRAC_PI_2)?;
    union(&half(&first, Vec3::Z)?, &half(&second, Vec3::X)?)
}

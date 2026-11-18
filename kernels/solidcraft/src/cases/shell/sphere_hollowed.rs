//! SolidCraft's Shell (`shell`) with no face to open: the ball hollowed to a wall 1 thick round a
//! closed cavity.
use crate::solids::*;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    solidcraft_kernel::shell(&ball, &[], 1.0)
}

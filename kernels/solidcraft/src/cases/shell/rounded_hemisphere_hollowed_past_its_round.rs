//! SolidCraft's Shell (`shell`) with no face to open: the rounded half ball hollowed to a wall 2
//! thick round a closed cavity.
use crate::solids::*;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    // everything below the plane z = 0 cut away
    let half = cut(&ball, &block((-10.0, -10.0, -10.0), (10.0, 10.0, 0.0))?)?;
    // the rim, where the flat meets the ball, picked at a point on it
    let rounded = solidcraft_kernel::fillet(&half, &[v3(0.0, 5.0, 0.0)], 1.0)?;
    solidcraft_kernel::shell(&rounded, &[], 2.0)
}

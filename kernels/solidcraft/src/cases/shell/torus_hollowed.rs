//! SolidCraft's Shell (`shell`) with no face to open: the torus hollowed to a wall 0.5 thick round
//! a closed cavity.
use crate::solids::*;

pub fn build() -> R<Body> {
    let torus = solidcraft_kernel::torus(Vec3::ZERO, 6.0, 2.0)?;
    solidcraft_kernel::shell(&torus, &[], 0.5)
}

//! SolidCraft's Shell (`shell`) with no face to open: the box hollowed to a wall 1 thick round a
//! closed cavity.
use crate::solids::*;

pub fn build() -> R<Body> {
    let b = block((0.0, 0.0, 0.0), (10.0, 8.0, 6.0))?;
    solidcraft_kernel::shell(&b, &[], 1.0)
}

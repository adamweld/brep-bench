//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them.
use crate::solids::*;

pub fn build() -> R<Body> {
    let c = zcyl(0.0, 0.0, 6.0, 0.0, 10.0)?;
    // the bottom edge, picked at a point on it
    let puck = solidcraft_kernel::fillet(&c, &[v3(0.0, 6.0, 0.0)], 2.0)?;
    solidcraft_kernel::shell(&puck, &[v3(1.0, 1.0, 10.0)], 3.0)
}

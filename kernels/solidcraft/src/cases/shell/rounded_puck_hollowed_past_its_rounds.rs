//! SolidCraft's Shell (`shell`) with no face to open: the rounded puck hollowed to a wall 3 thick
//! round a closed cavity.
use crate::solids::*;

pub fn build() -> R<Body> {
    let c = zcyl(0.0, 0.0, 6.0, 0.0, 10.0)?;
    // both circular edges, each picked at a point on it
    let puck = solidcraft_kernel::fillet(&c, &[v3(0.0, 6.0, 0.0), v3(0.0, 6.0, 10.0)], 2.0)?;
    solidcraft_kernel::shell(&puck, &[], 3.0)
}

//! SolidCraft's Shell (`shell`) with no face to open: the rounded frustum hollowed to a wall 2
//! thick round a closed cavity.
use crate::solids::*;

/// The kernel has no cone: SolidCraft's own cone feature revolves the half section, a trapezoid
/// with an edge on the axis, and so does this.
pub fn build() -> R<Body> {
    let frustum = revolve_z(polygon(&[(0.0, 0.0), (6.0, 0.0), (3.0, 6.0), (0.0, 6.0)]), 360.0)?;
    // both rims, the circles of radius 6 at z = 0 and 3 at z = 6, each picked at a point on it
    let rims = [v3(0.0, 6.0, 0.0), v3(0.0, 3.0, 6.0)];
    let rounded = solidcraft_kernel::fillet(&frustum, &rims, 1.5)?;
    solidcraft_kernel::shell(&rounded, &[], 2.0)
}

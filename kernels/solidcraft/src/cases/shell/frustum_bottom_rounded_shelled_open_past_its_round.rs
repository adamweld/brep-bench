//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them.
use crate::solids::*;

/// The kernel has no cone: SolidCraft's own cone feature revolves the half section, a trapezoid
/// with an edge on the axis, and so does this.
pub fn build() -> R<Body> {
    let frustum = revolve_z(polygon(&[(0.0, 0.0), (6.0, 0.0), (3.0, 6.0), (0.0, 6.0)]), 360.0)?;
    // the bottom rim, the circle of radius 6 at z = 0, picked at a point on it
    let rounded = solidcraft_kernel::fillet(&frustum, &[v3(0.0, 6.0, 0.0)], 1.5)?;
    solidcraft_kernel::shell(&rounded, &[v3(1.0, 1.0, 6.0)], 2.0)
}

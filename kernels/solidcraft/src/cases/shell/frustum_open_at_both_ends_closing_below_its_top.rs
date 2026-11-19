//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them.
use crate::solids::*;

/// solidcraft-kernel has no cone: SolidCraft's own cone feature revolves the half section, a trapezoid
/// with an edge on the axis, and so does this.
pub fn build() -> R<Body> {
    let frustum = revolve_z(polygon(&[(0.0, 0.0), (6.0, 0.0), (3.0, 4.0), (0.0, 4.0)]), 360.0)?;
    // the top and the base, each picked off the axis
    solidcraft_kernel::shell(&frustum, &[v3(1.0, 1.0, 4.0), v3(1.0, 1.0, 0.0)], 2.7)
}

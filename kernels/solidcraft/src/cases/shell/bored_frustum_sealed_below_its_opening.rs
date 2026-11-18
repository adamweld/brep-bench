//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them.
use crate::solids::*;

/// The kernel has no cone: SolidCraft's own cone feature revolves the half section, and so does
/// this, the section a trapezoid kept off the axis by the bore's radius.
pub fn build() -> R<Body> {
    let bored = revolve_z(polygon(&[(1.0, 0.0), (6.0, 0.0), (3.0, 4.0), (1.0, 4.0)]), 360.0)?;
    // the top, the annulus between radii 1 and 3, picked at radius 2
    solidcraft_kernel::shell(&bored, &[v3(0.0, 2.0, 4.0)], 1.0)
}

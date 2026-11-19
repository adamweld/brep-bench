//! A block, the box from the origin to (20, 10, 10), the edge where its top meets its side
//! x = 20 beveled 3 back on each face; then the bevel deleted, the top and the side run on until
//! they meet in the edge again: SolidCraft's Delete Face, healed by the planes beside the gap.
use crate::shared::edit::faces_nearest;
use crate::solids::*;

pub fn build() -> R<Body> {
    let edge = v3(20.0, 5.0, 10.0);
    let sharp = block((0.0, 0.0, 0.0), (20.0, 10.0, 10.0))?;
    let beveled = solidcraft_kernel::chamfer(&sharp, &[edge], 3.0)?;
    // the bevel: the face nearest where the edge was
    solidcraft_kernel::delete_faces(&beveled, &faces_nearest(&beveled, &[edge])?)
}

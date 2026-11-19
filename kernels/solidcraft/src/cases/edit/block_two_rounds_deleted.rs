//! A block, the box from the origin to (20, 10, 10), the two long edges of its top, at y = 0 and
//! y = 10, rounded to radius 2; then both rounds deleted together, the top and the sides run on
//! until they meet in the edges again.
use crate::shared::edit::faces_nearest;
use crate::solids::*;

pub fn build() -> R<Body> {
    let edges = [v3(10.0, 0.0, 10.0), v3(10.0, 10.0, 10.0)];
    let sharp = block((0.0, 0.0, 0.0), (20.0, 10.0, 10.0))?;
    let rounded = solidcraft_kernel::fillet(&sharp, &edges, 2.0)?;
    // the rounds: the faces nearest where the edges were
    solidcraft_kernel::delete_faces(&rounded, &faces_nearest(&rounded, &edges)?)
}

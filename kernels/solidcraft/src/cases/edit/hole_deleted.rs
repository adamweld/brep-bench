//! A block, the box from the origin to (20, 10, 10), drilled right through along z by a hole of
//! radius 3 on the line through (10, 5); then the hole's wall deleted, the top and the bottom
//! closing over the openings.
use crate::shared::edit::faces_nearest;
use crate::solids::*;

pub fn build() -> R<Body> {
    let drilled = cut(&block((0.0, 0.0, 0.0), (20.0, 10.0, 10.0))?, &zcyl(10.0, 5.0, 3.0, -1.0, 11.0)?)?;
    // the wall, by a point on it
    solidcraft_kernel::delete_faces(&drilled, &faces_nearest(&drilled, &[v3(10.0, 8.0, 5.0)])?)
}

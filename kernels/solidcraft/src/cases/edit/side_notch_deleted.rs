//! A block, the box from the origin to (20, 10, 10), with a half-round notch in its side y = 0:
//! the cylinder of radius 3 about the vertical line through (10, 0) cut away; then the notch's
//! wall deleted, the side running on across it and the top and bottom closing over it.
use crate::shared::edit::faces_nearest;
use crate::solids::*;

pub fn build() -> R<Body> {
    let notched = cut(&block((0.0, 0.0, 0.0), (20.0, 10.0, 10.0))?, &zcyl(10.0, 0.0, 3.0, -1.0, 11.0)?)?;
    // the wall, by points round it: the kernel's cylinder starts its circle at 0.6131 radians
    // from +x, so the half cylinder is two faces
    let wall: Vec<Vec3> = [0.3f64, 1.6, 2.9].iter().map(|a| v3(10.0 + 3.0 * a.cos(), 3.0 * a.sin(), 5.0)).collect();
    solidcraft_kernel::delete_faces(&notched, &faces_nearest(&notched, &wall)?)
}

//! An L-block, the outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) in the xy
//! plane extruded 8 along +z, its inside corner along x = 5, y = 5 rounded to radius 2; then the
//! round deleted, the faces x = 5 and y = 5 run back until they meet in the sharp corner again.
use crate::shared::edit::faces_nearest;
use crate::solids::*;

pub fn build() -> R<Body> {
    let outline = polygon(&[(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)]);
    let xy = Plane { origin: Vec3::ZERO, x: Vec3::X, y: Vec3::Y };
    let l_block = extrude(&xy, outline, 0.0, 8.0)?;
    let corner = v3(5.0, 5.0, 4.0);
    let rounded = solidcraft_kernel::fillet(&l_block, &[corner], 2.0)?;
    // the round: the face nearest where the corner was
    solidcraft_kernel::delete_faces(&rounded, &faces_nearest(&rounded, &[corner])?)
}

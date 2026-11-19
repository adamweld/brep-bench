//! An L-block, the outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) in the xy
//! plane extruded 8 along +z; then its step, the riser x = 5 and the tread y = 5, deleted
//! together: the far end y = 15 and the far side x = 20 run on until they meet, and the top and
//! the bottom close in over the corner.
use crate::shared::edit::faces_nearest;
use crate::solids::*;

pub fn build() -> R<Body> {
    let outline = polygon(&[(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)]);
    let xy = Plane { origin: Vec3::ZERO, x: Vec3::X, y: Vec3::Y };
    let l_block = extrude(&xy, outline, 0.0, 8.0)?;
    // the riser and the tread, by a point on each
    let step = faces_nearest(&l_block, &[v3(5.0, 10.0, 4.0), v3(12.0, 5.0, 4.0)])?;
    solidcraft_kernel::delete_faces(&l_block, &step)
}

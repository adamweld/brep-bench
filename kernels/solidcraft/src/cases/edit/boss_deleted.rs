//! A plate, the cylinder of radius 8 on the z axis from z = -4 to z = 0, and on it a boss, the
//! cylinder of radius 3 from z = 0 to z = 8; then the boss's side and top deleted, the plate's
//! top closing over the circle at its foot.
use crate::shared::edit::faces_nearest;
use crate::solids::*;

pub fn build() -> R<Body> {
    let bossed = union(&zcyl(0.0, 0.0, 8.0, -4.0, 0.0)?, &zcyl(0.0, 0.0, 3.0, 0.0, 8.0)?)?;
    // the side and the top, by a point on each
    let boss = faces_nearest(&bossed, &[v3(0.0, 3.0, 4.0), v3(0.0, 0.0, 8.0)])?;
    solidcraft_kernel::delete_faces(&bossed, &boss)
}

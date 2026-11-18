//! SolidCraft's Draft (`draft`) tilts the faces picked by points on them about their lines on the
//! neutral plane, leaning in along the pull. An L-block with arms 5 wide, 8 high, its six sides
//! drafted by 30° about its floor: each arm's sides meet in a ridge at 2.5 / tan 30°, below the
//! top, which the draft consumes.
use crate::solids::*;

pub fn build() -> R<Body> {
    let outline = [(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)];
    let block = extrude(&Plane::XY, polygon(&outline), 0.0, 8.0)?;
    // the middle of each side
    let sides = [
        v3(10.0, 0.0, 4.0),
        v3(20.0, 2.5, 4.0),
        v3(12.5, 5.0, 4.0),
        v3(5.0, 10.0, 4.0),
        v3(2.5, 15.0, 4.0),
        v3(0.0, 7.5, 4.0),
    ];
    solidcraft_kernel::draft(&block, &sides, &Plane::XY, Vec3::Z, 30f64.to_radians())
}

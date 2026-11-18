//! SolidCraft's Draft (`draft`) tilts the faces picked by points on them about their lines on the
//! neutral plane, leaning in along the pull. A T-block 8 high, its bar 20 by 4 and its stem 6
//! wide, its eight sides drafted by 45° about its floor: the bar's long sides meet in a ridge at 2
//! and the stem's at 3, below the top, which the draft consumes.
use crate::solids::*;

pub fn build() -> R<Body> {
    let outline =
        [(0.0, 0.0), (20.0, 0.0), (20.0, 4.0), (13.0, 4.0), (13.0, 16.0), (7.0, 16.0), (7.0, 4.0), (0.0, 4.0)];
    let block = extrude(&Plane::XY, polygon(&outline), 0.0, 8.0)?;
    // the middle of each side
    let sides = [
        v3(10.0, 0.0, 4.0),
        v3(20.0, 2.0, 4.0),
        v3(16.5, 4.0, 4.0),
        v3(13.0, 10.0, 4.0),
        v3(10.0, 16.0, 4.0),
        v3(7.0, 10.0, 4.0),
        v3(3.5, 4.0, 4.0),
        v3(0.0, 2.0, 4.0),
    ];
    solidcraft_kernel::draft(&block, &sides, &Plane::XY, Vec3::Z, 45f64.to_radians())
}

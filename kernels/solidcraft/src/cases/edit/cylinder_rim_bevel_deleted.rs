//! A cylinder of radius 6 on the z axis from z = 0 to z = 10, its top rim beveled 2 back on each
//! face: the section through (r, z) = (0, 0), (6, 0), (6, 8), (4, 10), (0, 10) turned about z.
//! Then the bevel, the cone from (6, 8) to (4, 10), deleted: the top and the side run on until
//! they meet in the circle of radius 6 at z = 10.
use crate::bench::polygon;
use crate::shared::defeature::delete_faces;
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::Body;
use std::f64::consts::TAU;

pub fn build() -> Result<Body, Refusal> {
    let section = polygon(&[
        [0.0, 0.0, 0.0],
        [6.0, 0.0, 0.0],
        [6.0, 0.0, 8.0],
        [4.0, 0.0, 10.0],
        [0.0, 0.0, 10.0],
    ]);
    let beveled = revolve(&section, [0.0; 3], [0.0, 0.0, 1.0], TAU, true)?;
    // the bevel, halfway up it
    delete_faces(&beveled, &[[0.0, 5.0, 9.0]])
}

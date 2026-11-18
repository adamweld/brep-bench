//! A cone frustum on the z axis, of radius 6 at z = 0, its side the cone 2r + z = 12, with its top
//! rim beveled: the section through (r, z) = (0, 0), (6, 0), (3.5, 5), (2, 6), (0, 6) turned about
//! z. Then the bevel, the steeper cone from (3.5, 5) to (2, 6), deleted: the top and the side run
//! on until they meet in the circle of radius 3 at z = 6.
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
        [3.5, 0.0, 5.0],
        [2.0, 0.0, 6.0],
        [0.0, 0.0, 6.0],
    ]);
    let beveled = revolve(&section, [0.0; 3], [0.0, 0.0, 1.0], TAU, true)?;
    // the bevel, halfway up it
    delete_faces(&beveled, &[[0.0, 2.75, 5.5]])
}

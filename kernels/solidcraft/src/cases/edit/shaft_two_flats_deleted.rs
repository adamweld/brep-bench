//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with two opposite
//! flats milled along its whole length where |x| > 4; then both flats deleted together, the
//! cylinder closing over them and the ends growing back to whole discs.
use crate::bench::cut_all;
use crate::shared::defeature::delete_faces;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let shaft = make_cylinder([0.0, 0.0, 0.0], 5.0, 20.0)?;
    let right = make_box([4.0, -6.0, -1.0], [2.0, 12.0, 22.0])?;
    let left = make_box([-6.0, -6.0, -1.0], [2.0, 12.0, 22.0])?;
    let milled = cut_all(shaft, [right, left])?;
    delete_faces(&milled, &[[4.0, 0.0, 10.0], [-4.0, 0.0, 10.0]])
}

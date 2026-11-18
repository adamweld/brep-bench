//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a flat milled
//! along its whole length where y < −4; then the flat deleted, the cylinder closing over it and
//! the ends growing back to whole discs.
use crate::bench::cut;
use crate::shared::defeature::delete_faces;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let shaft = make_cylinder([0.0, 0.0, 0.0], 5.0, 20.0)?;
    let below = make_box([-6.0, -6.0, -1.0], [12.0, 2.0, 22.0])?;
    let milled = cut(shaft, below)?;
    delete_faces(&milled, &[[0.0, -4.0, 10.0]])
}

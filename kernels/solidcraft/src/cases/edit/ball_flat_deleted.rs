//! A ball of radius 5 about the origin with its top cut flat at z = 3, leaving a disc of radius 4;
//! then the flat deleted, the ball's sphere running on over it until the ball is whole again.
use crate::bench::cut;
use crate::shared::defeature::delete_faces;
use crate::Refusal;
use solidcore_ops::{make_box, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    let above = make_box([-6.0, -6.0, 3.0], [12.0, 12.0, 3.0])?;
    let flat = cut(ball, above)?;
    delete_faces(&flat, &[[0.0, 0.0, 3.0]])
}

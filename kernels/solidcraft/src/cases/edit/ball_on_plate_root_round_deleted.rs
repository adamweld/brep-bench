//! A plate, the disk of radius 8 about the z axis from z = -2 to z = 0, joined to the ball of
//! radius 3 about (0, 0, 1), which stands out of the plate's top through the circle of radius
//! √8; that concave edge rounded to radius 1, then the round deleted, the top and the ball run
//! on until they meet in the circle again.
use crate::bench::fuse;
use crate::shared::edit::{delete_rounds, round_the_rim, rounded};
use crate::Refusal;
use solidcore_ops::{make_cylinder, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let joined = fuse(make_cylinder([0.0, 0.0, -2.0], 8.0, 2.0)?, make_sphere([0.0, 0.0, 1.0], 3.0)?)?;
    // the edge and then its round, by points all round it: the ball's faces split it
    let root = round_the_rim(8f64.sqrt(), 0.0);
    let joined = rounded(&joined, &root, 1.0)?;
    delete_rounds(&joined, &root)
}

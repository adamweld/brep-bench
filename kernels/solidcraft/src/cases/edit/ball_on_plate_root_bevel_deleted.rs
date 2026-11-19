//! A plate, the disk of radius 8 about the z axis from z = −2 to z = 0, the ball of radius 3
//! about (0, 0, 1) joined to it, and the concave edge where they meet beveled by joining the
//! frustum from radius 4 at z = 0 to radius 3 at z = 1. Then the bevel, that frustum's cone,
//! deleted: the plate's top runs on inward and the ball down from its equator until they meet in
//! the circle of radius √8 at z = 0.
use crate::bench::fuse_all;
use crate::shared::defeature::delete_faces;
use crate::Refusal;
use solidcore_ops::{make_cone, make_cylinder, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let plate = make_cylinder([0.0, 0.0, -2.0], 8.0, 2.0)?;
    let ball = make_sphere([0.0, 0.0, 1.0], 3.0)?;
    let bevel = make_cone([0.0, 0.0, 0.0], 4.0, 3.0, 1.0)?;
    let part = fuse_all([plate, ball, bevel])?;
    // the bevel, halfway up it
    delete_faces(&part, &[[0.0, 3.5, 0.5]])
}

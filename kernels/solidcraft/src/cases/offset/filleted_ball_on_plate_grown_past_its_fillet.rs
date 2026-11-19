//! A plate, the cylinder of radius 8 on the z axis from z = −2 to z = 0, joined to the ball of
//! radius 3 about (0, 0, 1), the concave edge where they meet filleted to radius 1, grown by 2:
//! every point within 2 of it, the kernel's offset with its convex edges rounded.
use super::rounded_puck_shrunk_by_its_rounds::round_circles;
use crate::bench::fuse;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_cylinder, make_sphere, Body};

/// The plate and ball joined, the circle of radius √8 at z = 0 where the ball leaves the plate's
/// top filleted to `radius`.
pub fn filleted_ball_on_plate(radius: f64) -> Result<Body, Refusal> {
    let plate = make_cylinder([0.0, 0.0, -2.0], 8.0, 2.0)?;
    let ball = make_sphere([0.0, 0.0, 1.0], 3.0)?;
    round_circles(fuse(plate, ball)?, &[(8f64.sqrt(), 0.0)], radius)
}

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&filleted_ball_on_plate(1.0)?, 2.0)
}

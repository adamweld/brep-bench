//! A plate, the cylinder of radius 8 on the z axis from z = −2 to z = 0, joined to a boss, the
//! cone frustum from radius 4 at z = 0 to radius 1 at z = 6, the concave edge where they meet
//! filleted to radius 1, grown by 1: every point within 1 of it, the kernel's offset with its
//! convex edges rounded.
use super::rounded_puck_shrunk_by_its_rounds::round_circles;
use crate::bench::fuse;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_cone, make_cylinder, Body};

/// The plate and boss joined, the circle of radius 4 at z = 0 where the boss stands on the
/// plate's top filleted to `radius`.
pub fn filleted_cone_boss(radius: f64) -> Result<Body, Refusal> {
    let plate = make_cylinder([0.0, 0.0, -2.0], 8.0, 2.0)?;
    let boss = make_cone([0.0, 0.0, 0.0], 4.0, 1.0, 6.0)?;
    round_circles(fuse(plate, boss)?, &[(4.0, 0.0)], radius)
}

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&filleted_cone_boss(1.0)?, 1.0)
}

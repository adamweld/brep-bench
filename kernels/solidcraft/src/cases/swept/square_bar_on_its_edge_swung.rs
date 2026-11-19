//! The solid a square bar fills as it is swung a quarter turn about the z axis, right-handed: the
//! prism from z = 0 to 3 on the square with corners (4, 0), (6, 2), (8, 0) and (6, −2).
//!
//! solidcraft-kernel has no sweep of a solid, so the swing is built by elementary geometry. The
//! square is symmetric about the x axis and convex, so it meets each circle about the z axis in
//! one arc holding the circle's point on the x axis, and turned a quarter turn it passes over that
//! arc where it starts, the quarter circle on from there, and the arc where it stops. So the
//! sweep is the bar's section by the half-plane at 0°, the rectangle 4 ≤ x ≤ 8, 0 ≤ z ≤ 3,
//! revolved the quarter turn, with the bar's half behind that section where it starts (y ≤ 0)
//! and its half ahead of it where it stops.
use super::drum_swung_partway_into_ring_sector::turned;
use crate::solids::*;

pub fn build() -> R<Body> {
    let level = Plane { origin: Vec3::ZERO, x: Vec3::X, y: Vec3::Y };
    let behind = extrude(&level, polygon(&[(4.0, 0.0), (6.0, -2.0), (8.0, 0.0)]), 0.0, 3.0)?;
    let ahead = extrude(&level, polygon(&[(4.0, 0.0), (8.0, 0.0), (6.0, 2.0)]), 0.0, 3.0)?;
    let fan = revolve_z(rect(4.0, 0.0, 8.0, 3.0), 90.0)?;
    union(&union(&behind, &fan)?, &turned(&ahead, 90.0)?)
}

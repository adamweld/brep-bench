//! A plate, the box from (0, −5, 0) to (20, 5, 4), with a hole bored through it by a drill: the
//! solid of revolution about the vertical line through (10, 0) of a cylinder of radius 2 from
//! z = 6 + 2/tan 59° up to z = 14 ended below in a cone of 118° whose point is (10, 0, 6), slid
//! without turning straight down along its axis by 20.
//!
//! solidcraft-kernel has no sweep of a solid, so the drill's sweep is built by elementary
//! geometry. A slide along the axis commutes with the turns about it, so the sweep is the solid
//! of revolution of the drill's half section slid down 20. The section's width only grows up to
//! the top of the cone and then stays 2, so slid it covers the drill's cone where it stops (its
//! point at z = −14) and the full width from there up to z = 14.
use crate::solids::*;
use std::f64::consts::TAU;

pub fn build() -> R<Body> {
    let plate = block((0.0, -5.0, 0.0), (20.0, 5.0, 4.0))?;
    let cone = 2.0 / 59f64.to_radians().tan();
    // the half section in (r, z), drawn on the plane through the axis
    let section = polygon(&[(0.0, -14.0), (2.0, -14.0 + cone), (2.0, 14.0), (0.0, 14.0)]);
    let plane = Plane { origin: v3(10.0, 0.0, 0.0), x: Vec3::X, y: Vec3::Z };
    let swept = one(solidcraft_kernel::revolve(&plane, &[section], Vec2::ZERO, Vec2::Y, TAU)?)?;
    cut(&plate, &swept)
}

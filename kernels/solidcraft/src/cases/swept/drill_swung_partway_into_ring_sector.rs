//! A ring sector, everything between 8 and 12 from the z axis with |z| ≤ 1 and polar angle from
//! 0° to 60°, notched by a drill swung about the z axis from −140° to +30°: about the vertical
//! line through (10, 0), a cylinder of radius 1 up to z = 3, ended below in a cone of 118° whose
//! point is (10, 0, −0.8).
//!
//! solidcraft-kernel has no sweep of a solid, so the swing is built by elementary geometry
//! ([`swung_partway`]): the drill meets each circle about the z axis in one arc holding the
//! circle's point at 0°, so its section through its axis there is its shadow, revolved through
//! the turn, with the drill's half ahead of that section where it stops.
use super::drum_swung_partway_into_ring_sector::{half_ahead, swung_partway};
use crate::solids::*;

pub fn build() -> R<Body> {
    // the rim where the cone, each side 59° from the axis, reaches the cylinder's radius 1
    let rim = -0.8 + 1.0 / 59f64.to_radians().tan();
    let section = polygon(&[(10.0, -0.8), (11.0, rim), (11.0, 3.0), (9.0, 3.0), (9.0, rim)]);
    let half = polygon(&[(10.0, -0.8), (11.0, rim), (11.0, 3.0), (10.0, 3.0)]);
    swung_partway(section, Some(half_ahead(half)?))
}

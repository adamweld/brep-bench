//! A ring sector, everything between 8 and 12 from the z axis with |z| ≤ 1 and polar angle from
//! 0° to 60°, notched by a half drum swung about the z axis from −140° to +30°: the half with
//! y ≤ 0 of the solid cylinder of radius 1 about the vertical line through (10, 0) from z = −0.5
//! to 0.5, its flat face leading.
//!
//! solidcraft-kernel has no sweep of a solid, so the swing is built by elementary geometry
//! ([`swung_partway`]): the half drum meets each circle about the z axis in one arc ending at the
//! circle's point at 0°, on its flat face, so its section by the half-plane at 0°, that face, is
//! its shadow, and nothing of it lies ahead of the face. What it sweeps in the block is the face
//! revolved through the turn.
use super::drum_swung_partway_into_ring_sector::swung_partway;
use crate::solids::*;

pub fn build() -> R<Body> {
    swung_partway(rect(9.0, -0.5, 11.0, 0.5), None)
}

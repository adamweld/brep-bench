//! A ring sector, everything between 8 and 12 from the z axis with |z| ≤ 1 and polar angle from
//! 0° to 60°, notched by a drum, the solid cylinder of radius 1 about the vertical line through
//! (10, 0) from z = −0.5 to 0.5, swung about the z axis from −140° to +30°.
//!
//! solidcraft-kernel has no sweep of a solid, so the swing is built by elementary geometry
//! ([`swung_partway`]): the drum meets each circle about the z axis in one arc holding the
//! circle's point at 0°, so its section by the half-plane at 0°, the rectangle 9 ≤ x ≤ 11,
//! |z| ≤ 0.5, is its shadow, and what it sweeps is that rectangle revolved through the turn with
//! the drum where it starts and where it stops. Where it starts it is clear of the block, and
//! where it stops only its half ahead of the half-plane through its axis lies beyond the
//! revolved rectangle.
use crate::solids::*;
use std::f64::consts::PI;

/// The block the swung tools notch: the rectangle 8 ≤ x ≤ 12, |z| ≤ 1 revolved from 0° to 60°.
pub fn ring_sector() -> R<Body> {
    revolve_z(rect(8.0, -1.0, 12.0, 1.0), 60.0)
}

/// Where the swung tools start and stop, in degrees about the z axis.
pub const START: f64 = -140.0;
pub const STOP: f64 = 30.0;

/// `body` turned by `degrees` about the z axis, right-handed.
pub fn turned(body: &Body, degrees: f64) -> R<Body> {
    solidcraft_kernel::transform(body, Vec3::ZERO, Vec3::ZERO, Vec3::Z, degrees.to_radians())
}

/// The ring sector less what a tool passes through as it is swung about the z axis from
/// [`START`] to [`STOP`], given as it is drawn, at 0°. A tool that meets each circle about the
/// z axis in arcs whose gaps are narrower than the turn passes over, on each circle, everything
/// from its first point where it starts to its last point where it stops. When every circle it
/// meets holds a point of its section by the half-plane at 0° (`shadow`, in x and z), that is
/// the section revolved through the turn, with the tool where it starts and where it stops,
/// each beyond the section's two ends. Where it starts the tool is clear of the block; where it
/// stops, `lead` is what of it lies ahead of the half-plane, drawn at 0° (`None`: nothing).
pub fn swung_partway(shadow: Region2, lead: Option<Body>) -> R<Body> {
    let mut swept = turned(&revolve_z(shadow, STOP - START)?, START)?;
    if let Some(lead) = lead {
        swept = union(&swept, &turned(&lead, STOP)?)?;
    }
    cut(&ring_sector()?, &swept)
}

/// The half of a tool of revolution about the vertical line through (10, 0) ahead of the
/// half-plane through its axis (towards +y, where it moves): `half`, its section in x and z on
/// the axis's outer side, revolved half a turn about it. (The kernel turns a section on `xz`
/// towards −y for a positive angle.)
pub fn half_ahead(half: Region2) -> R<Body> {
    one(solidcraft_kernel::revolve(&xz(), &[half], v2(10.0, 0.0), Vec2::Y, -PI)?)
}

pub fn build() -> R<Body> {
    swung_partway(rect(9.0, -0.5, 11.0, 0.5), Some(half_ahead(rect(10.0, -0.5, 11.0, 0.5))?))
}

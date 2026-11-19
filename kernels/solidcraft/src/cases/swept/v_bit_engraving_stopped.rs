//! A block, the box from (0, −10, −10) to (30, 10, 0), with a line engraved into its top by a V
//! bit: the solid cone about the vertical line through (5, 0), its point at (5, 0, −2), opening
//! upward at 90° to a flat top of radius 5 at z = 3, slid without turning along x by 15.
//!
//! solidcraft-kernel has no sweep of a solid, so the bit's sweep is built by elementary geometry.
//! Split by the plane x = 5 through its axis, square to the slide, the bit's back half leads with
//! its flat cut face only (its cone faces back, and its top moves in its own plane), so it sweeps
//! itself and that triangle slid 15; the front half likewise sweeps the triangle and itself where
//! it stops. So the sweep is the back half where the bit starts, the triangle on x = 5 extruded
//! along x by 15, and the front half where it stops.
use crate::solids::*;
use std::f64::consts::TAU;

/// The half of the bit about the vertical line through (`x`, 0) on the side `outward` points to.
fn half_bit(x: f64, outward: Vec3) -> R<Body> {
    let plane = Plane { origin: v3(x, 0.0, 0.0), x: Vec3::X, y: Vec3::Z };
    let section = polygon(&[(0.0, -2.0), (5.0, 3.0), (0.0, 3.0)]);
    let bit = one(solidcraft_kernel::revolve(&plane, &[section], Vec2::ZERO, Vec2::Y, TAU)?)?;
    let cap = Plane::from_normal(plane.origin, outward)
        .ok_or_else(|| KernelError::Invalid("the half bit's direction".into()))?;
    solidcraft_kernel::half_space(&bit, &cap, true)?
        .0
        .ok_or_else(|| KernelError::Failed("the half bit is empty".into()))
}

pub fn build() -> R<Body> {
    let stock = block((0.0, -10.0, -10.0), (30.0, 10.0, 0.0))?;
    // the bit's section through its axis, in (y, z) on the plane x = 5, extruded along +x
    let section = polygon(&[(0.0, -2.0), (5.0, 3.0), (-5.0, 3.0)]);
    let plane = Plane { origin: v3(5.0, 0.0, 0.0), x: Vec3::Y, y: Vec3::Z };
    let mut swept = extrude(&plane, section, 0.0, 15.0)?;
    swept = union(&swept, &half_bit(5.0, -Vec3::X)?)?;
    swept = union(&swept, &half_bit(20.0, Vec3::X)?)?;
    cut(&stock, &swept)
}

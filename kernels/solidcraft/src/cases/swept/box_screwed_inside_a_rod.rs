//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a sealed helical
//! tunnel: what the box 2.5 ≤ x ≤ 3.5, −0.5 ≤ y ≤ 0.5, 2 ≤ z ≤ 3 passes through as the screw about
//! the z axis that rises 4 a turn carries it from a turn of 0° to one of 720°.
//!
//! solidcraft-kernel has no sweep of a solid, so the box's sweep is built by elementary geometry
//! from the kernel's helical sweep of planar regions (`sweep_helix`). On each cylinder about the
//! axis the screw slides the cylinder along itself, and unrolled, the box's trace there is one
//! rectangle or two, each from z = 2 to z = 3 between two angles. A rectangle slid along a line
//! passes over itself at each end and over what is swept by any path across it from its top edge
//! down to its bottom edge, joined to both edges. Here those are the box's faces z = 2 and z = 3,
//! and its sections by half-planes through the axis: the one at angle 0 crosses the box's trace
//! out to radius 3.5, and those through the box's outer upright edges, at ±atan(0.5/3.5), cross
//! each of the two rectangles beyond it. Every one of them crosses the screw's motion, never
//! moving along itself, so its screw sweep is what it passes through. With the box where it
//! starts and where it ends, these are the tunnel.
use super::square_bar_screwed_through_rod::{screw, screwed};
use crate::solids::*;

pub fn build() -> R<Body> {
    let (lead, start, stop) = (4.0, 0.0, 720.0);
    let mut tunnel = vec![
        block((2.5, -0.5, 2.0), (3.5, 0.5, 3.0))?,
        block((2.5, -0.5, 10.0), (3.5, 0.5, 11.0))?,
    ];
    for z in [2.0, 3.0] {
        let level = Plane { origin: v3(0.0, 0.0, z), x: Vec3::X, y: Vec3::Y };
        tunnel.push(screw(&level, &rect(2.5, -0.5, 3.5, 0.5), lead, start, stop)?);
    }
    tunnel.push(screw(&xz(), &rect(2.5, 2.0, 3.5, 3.0), lead, start, stop)?);
    // the half-planes through the corner edges (3.5, ±0.5), from the face x = 2.5 to the corner
    let corner = (3.5f64 * 3.5 + 0.5 * 0.5).sqrt();
    let angle = (0.5f64 / 3.5).atan().to_degrees();
    for side in [-1.0, 1.0] {
        let half = screwed(&xz(), 0.0, side * angle);
        let inner = 2.5 * corner / 3.5;
        tunnel.push(screw(&half, &rect(inner, 2.0, corner, 3.0), lead, start, stop)?);
    }
    let mut rod = zcyl(0.0, 0.0, 5.0, 0.0, 20.0)?;
    for piece in &tunnel {
        rod = cut(&rod, piece)?;
    }
    Ok(rod)
}

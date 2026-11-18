//! A plate (the cylinder of radius 8 from z = −6 to 0) with a ball of radius 5 about the origin
//! standing in it, the concave circle where the ball meets the plate's top chamfered by a distance
//! and an angle: 2 out from the edge on the plate, the chamfer rising at 45° from the plate back to
//! the ball, filling the groove.
//!
//! The plate is a circle extruded and the ball cadrs's sphere primitive, united; the concave edge
//! goes into one Chamfer feature of a distance and an angle, the distance on the plate's top.
use crate::bench::{ball, cylinder, p3, union, v3};
use crate::shared::chamfer::distance_angle_off_ball;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const ON_PLATE: f64 = 2.0;
const ANGLE_DEGREES: f64 = 45.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = cylinder(k, p3(0.0, 0.0, -6.0), v3(0.0, 0.0, 1.0), 8.0, 6.0)?;
    let b = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let body = union(k, plate, b)?;
    // The edge through (0, −5, 0); the ball is about the origin, through its top.
    distance_angle_off_ball(k, body, ON_PLATE, ANGLE_DEGREES.to_radians())
}

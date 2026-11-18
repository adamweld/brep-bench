//! A ball in a conical seat, the rim between them chamfered by a distance of 2 and an angle of 30°.
//! The plate is cadrs's cylinder less its cone primitive (apex at (0, 0, 1), radius 8 at z = 5),
//! the ball its sphere primitive, united; the concave edge goes into one Chamfer feature of a
//! distance and an angle, the distance on the seat's cone.
use crate::bench::{ball, cone, cut, cylinder, p3, union, v3};
use crate::shared::chamfer::distance_angle_off_ball;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const UP_CONE: f64 = 2.0;
const ANGLE_DEGREES: f64 = 30.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let plate = cylinder(k, p3(0.0, 0.0, -6.0), z, 8.0, 11.0)?;
    let hollow = cone(k, p3(0.0, 0.0, 1.0), z, 0.0, 8.0, 4.0)?;
    let seat = cut(k, plate, hollow)?;
    let b = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let body = union(k, seat, b)?;
    // The edge through (0, −4, 3); the ball is about the origin, through its top.
    distance_angle_off_ball(k, body, UP_CONE, ANGLE_DEGREES.to_radians())
}

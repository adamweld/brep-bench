//! The upper half of a ball of radius 5 about the origin, the circle where its flat face meets the
//! ball chamfered by a distance and an angle: 1 in from the rim on the flat face, the chamfer
//! rising at 45° from the flat until it meets the ball.
//!
//! The ball is cadrs's sphere primitive, kept above z = 0 by its common with a block; the rim goes
//! into one Chamfer feature of a distance and an angle, the distance on the flat face.
use crate::bench::{ball, block, common, p3};
use crate::shared::chamfer::distance_angle_off_ball;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const ON_FLAT: f64 = 1.0;
const ANGLE_DEGREES: f64 = 45.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let upper = block(k, p3(-20.0, -20.0, 0.0), p3(20.0, 20.0, 20.0))?;
    let hemisphere = common(k, b, upper)?;
    // The rim through (0, −5, 0); the ball is about the origin, through its top.
    distance_angle_off_ball(
        k,
        hemisphere,
        p3(0.0, -5.0, 0.0),
        p3(0.0, 0.0, 0.0),
        p3(0.0, 0.0, 5.0),
        ON_FLAT,
        ANGLE_DEGREES.to_radians(),
    )
}

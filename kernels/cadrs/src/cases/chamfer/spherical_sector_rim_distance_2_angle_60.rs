//! The spherical sector of the ball of radius 5 about the origin inside the cone with its apex at
//! the origin about the z axis, cos α = 3/5, its rim (the circle of radius 4 at z = 3) chamfered by
//! a distance and an angle: 2 down the cone from the rim, the chamfer leaving the cone's side at
//! 60° toward the ball.
//!
//! The sector is cadrs's sphere primitive in common with its cone primitive (apex at the origin,
//! radius 8 at z = 6); the rim goes into one Chamfer feature of a distance and an angle, the
//! distance on the cone.
use crate::bench::{ball, common, cone, p3, v3};
use crate::shared::chamfer::distance_angle_off_ball;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const DOWN_CONE: f64 = 2.0;
const ANGLE_DEGREES: f64 = 60.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let c = cone(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 0.0, 8.0, 6.0)?;
    let sector = common(k, b, c)?;
    distance_angle_off_ball(k, sector, DOWN_CONE, ANGLE_DEGREES.to_radians())
}

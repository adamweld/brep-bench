//! A frustum capped by a ball, its rim chamfered by a distance of 1 and an angle of 45°.
//! The frustum is cadrs's cone primitive (radius 2 at z = 0 to 4 at z = 3), the cap its sphere primitive less a block
//! below z = 3, united; the rim goes into one Chamfer feature of a distance and an angle, the distance on the cone.
use crate::bench::{ball, block, common, cone, p3, union, v3};
use crate::shared::chamfer::distance_angle_off_ball;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const DOWN_CONE: f64 = 1.0;
const ANGLE_DEGREES: f64 = 45.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let frustum = cone(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 2.0, 4.0, 3.0)?;
    let b = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let above = block(k, p3(-6.0, -6.0, 3.0), p3(6.0, 6.0, 6.0))?;
    let cap = common(k, b, above)?;
    let body = union(k, frustum, cap)?;
    // The rim through (0, −4, 3); the ball is about the origin, through its top.
    distance_angle_off_ball(
        k,
        body,
        p3(0.0, -4.0, 3.0),
        p3(0.0, 0.0, 0.0),
        p3(0.0, 0.0, 5.0),
        DOWN_CONE,
        ANGLE_DEGREES.to_radians(),
    )
}

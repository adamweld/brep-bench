//! A plate, the cylinder of radius 8 on the z axis from z = −6 to z = 0, joined to a ball of radius
//! 5 centred on the origin, whose upper half rises from the plate's top. The concave edge where the
//! ball meets the top, the circle of radius 5, chamfered by a distance and an angle: 2 out from the
//! edge on the plate's top, the chamfer rising from there at 45° to the top back toward the ball.
//! A concave chamfer adds material: it fills the groove between the plate and the ball.
//!
//! Built as `ball_on_plate_rim_distance_1_angle_50` builds it: the meridian's corner cut by a
//! distance and an angle, turned once about z.
use super::ball_on_plate_rim_distance_1_angle_50::ball_on_plate;
use cad_kernel::topo::Brep;

const DISTANCE: f64 = 2.;
const ANGLE: f64 = 45.;

pub fn build() -> Result<Brep, String> {
    ball_on_plate(DISTANCE, ANGLE)
}

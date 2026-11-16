//! A ball of radius 5 about the origin standing in a conical seat: the cylinder of radius 8 about
//! the z axis from z = −6 to z = 5, everything above the cone z = 1 + ρ/2 hollowed out of it,
//! joined to the ball. The seat's cone meets the ball in the circle of radius 4 at z = 3, a concave
//! edge, chamfered by a distance and an angle: 2 up the seat's cone from the edge, the chamfer
//! making 30° with the cone, rising from it back toward the ball. A concave chamfer adds material:
//! it fills the groove between the seat and the ball.
//!
//! Built as `ball_in_conical_seat_distance_1_angle_45` builds it: the meridian's corner cut by a
//! distance and an angle, turned once about z.
use super::ball_in_conical_seat_distance_1_angle_45::ball_in_seat;
use cad_kernel::topo::Brep;

const DISTANCE: f64 = 2.;
const ANGLE: f64 = 30.;

pub fn build() -> Result<Brep, String> {
    ball_in_seat(DISTANCE, ANGLE)
}

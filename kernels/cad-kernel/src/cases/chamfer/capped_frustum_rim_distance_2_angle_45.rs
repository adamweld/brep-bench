//! A frustum on the z axis, widening upward from the disc of radius 2 at z = 0 to radius 4 at
//! z = 3, capped by the part of the ball of radius 5 about the origin above z = 3. Its rim, where
//! the ball meets the frustum's cone, chamfered by a distance and an angle: 2 down the cone from
//! the rim, the chamfer making 45° with the cone's side, turned toward the z axis.
//!
//! Built as `capped_frustum_rim_distance_1_angle_45` builds it: the meridian's corner cut by a
//! distance and an angle, turned once about z.
use super::capped_frustum_rim_distance_1_angle_45::capped_frustum;
use cad_kernel::topo::Brep;

const DISTANCE: f64 = 2.;
const ANGLE: f64 = 45.;

pub fn build() -> Result<Brep, String> {
    capped_frustum(DISTANCE, ANGLE)
}

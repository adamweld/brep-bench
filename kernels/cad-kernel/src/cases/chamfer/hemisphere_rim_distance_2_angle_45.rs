//! The upper half of a ball of radius 5 centred on the origin, the rim where its flat face meets
//! the ball chamfered by a distance and an angle: 2 in from the rim on the flat face, the chamfer
//! rising from there at 45° to the flat toward the ball. The chamfer is the cone that line sweeps
//! about the z axis; the flat shrinks to the disc of radius 3.
//!
//! Built as `hemisphere_rim_distance_1_angle_45` builds it: the meridian's corner cut by a distance
//! and an angle, turned once about z.
use super::hemisphere_rim_distance_1_angle_45::hemisphere;
use cad_kernel::topo::Brep;

const DISTANCE: f64 = 2.;
const ANGLE: f64 = 45.;

pub fn build() -> Result<Brep, String> {
    hemisphere(DISTANCE, ANGLE)
}

//! A spherical sector: the ball of radius 5 centred on the origin inside the cone from the origin
//! about the z axis, opening upward with half angle α, cos α = 3/5. Its rim, where the cone meets
//! the ball, chamfered by a distance and an angle: 3 down the cone from the rim, the chamfer making
//! 30° with the cone's side, turned toward the z axis into the sector.
//!
//! Built as `spherical_sector_rim_distance_1_angle_45` builds it: the sector's meridian, its corner
//! cut by a distance and an angle, turned once about z.
use super::spherical_sector_rim_distance_1_angle_45::sector;
use cad_kernel::topo::Brep;

const COS_ALPHA: f64 = 3. / 5.;
const DISTANCE: f64 = 3.;
const ANGLE: f64 = 30.;

pub fn build() -> Result<Brep, String> {
    sector(COS_ALPHA, DISTANCE, ANGLE)
}

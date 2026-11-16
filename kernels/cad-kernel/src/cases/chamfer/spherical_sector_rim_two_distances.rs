//! A spherical sector: the ball of radius 5 centred on the origin inside the cone from the origin
//! about the z axis, opening upward with half angle α, cos α = 3/5. Its rim, where the cone meets
//! the ball, chamfered with two distances: 2 down the cone and 1 on the ball, each a straight line
//! from the edge in the plane through the z axis. The chamfer is the cone between the two circles
//! it reaches.
//!
//! The sector's meridian, as `spherical_sector_rim_distance_1_angle_45` draws it, chamfered at its
//! corner by `corners::chamfer`, whose `Leg::FromEdge` is a straight chord from the corner, and
//! turned once about the z axis.
use super::hemisphere_rim_distance_1_angle_45::turned;
use super::spherical_sector_rim_distance_1_angle_45::{meridian, BALL, SIDE};
use cad_kernel::corners::{chamfer, Leg};
use cad_kernel::topo::Brep;

const COS_ALPHA: f64 = 3. / 5.;
const DOWN_CONE: f64 = 2.;
const ON_BALL: f64 = 1.;

const CHAMFER: u32 = 4;

pub fn build() -> Result<Brep, String> {
    let legs = [Leg::FromEdge(DOWN_CONE), Leg::FromEdge(ON_BALL)];
    let cut = chamfer(&meridian(COS_ALPHA)?, &[SIDE], &[BALL], legs, CHAMFER)?;
    turned(&cut)
}

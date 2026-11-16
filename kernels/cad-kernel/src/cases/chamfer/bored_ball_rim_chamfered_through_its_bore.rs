//! A ball of radius 5 centred on the origin, a hole of radius 2 bored right through it along the z
//! axis, meeting the ball in two circles of radius 2 at z = ±√21. The upper one chamfered with two
//! distances: 2√21 down the bore, its whole length, and 1 on the ball, each a straight line from
//! the edge in the plane through the z axis. The bore shrinks to the lower rim and is gone: the
//! hole is the cone from the lower rim up to the circle 1 from the upper one.
//!
//! The bored ball's meridian, as `bored_ball_rim_two_distances` draws it, chamfered at its upper
//! corner and turned once about the z axis. The chamfer consumes the bore, which the kernel's
//! `corners::chamfer` declines (and it would cut the lower rim too); so the corner's flat is taken
//! from `corners::flat` and the piece it cuts off taken away by a planar Boolean, as
//! `hemisphere_rim_chamfered_to_its_center` does.
use super::bored_ball_rim_two_distances::{bored, BALL, BORE};
use super::hemisphere_rim_chamfered_to_its_center::chamfered_at;
use super::hemisphere_rim_distance_1_angle_45::turned;
use cad_kernel::corners::Leg;
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const BORE_RADIUS: f64 = 2.;
const ON_BALL: f64 = 1.;

const CHAMFER: u32 = 3;

pub fn build() -> Result<Brep, String> {
    // the upper rim, at (2, √21), and the bore's length, from it to the lower rim
    let height = (RADIUS * RADIUS - BORE_RADIUS * BORE_RADIUS).sqrt();
    let down_bore = 2. * height;
    let legs = [(BORE, Leg::FromEdge(down_bore)), (BALL, Leg::FromEdge(ON_BALL))];
    let cut = chamfered_at(&bored()?, [BORE_RADIUS, height], legs, CHAMFER)?;
    turned(&cut)
}

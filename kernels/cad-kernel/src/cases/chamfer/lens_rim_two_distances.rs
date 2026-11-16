//! The lens common to two balls of radius 5, one centred at (0, 0, 3) and one at (0, 0, −3): its
//! upper face on the ball about (0, 0, −3), its lower face on the ball about (0, 0, 3), meeting in
//! its rim, the circle of radius 4 at z = 0. The rim chamfered with two distances: 2 on the upper
//! face and 1 on the lower, each a straight line from the edge in the plane through the z axis. The
//! chamfer is the cone between the two circles it reaches.
//!
//! The lens is a solid of revolution, so it is built as one: its meridian, the two balls' half
//! discs in common (a planar Boolean), chamfered at its corner by `corners::chamfer`, whose
//! `Leg::FromEdge` is a straight chord from the corner (a circle about it, met with each face's),
//! and turned once about the z axis.
use super::hemisphere_rim_distance_1_angle_45::turned;
use cad_kernel::corners::{chamfer, Leg};
use cad_kernel::planar::{boolean, Op, Region};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const CENTRE: f64 = 3.;
const ON_UPPER: f64 = 2.;
const ON_LOWER: f64 = 1.;

// the faces of the meridian, by tag (0 is none)
const UPPER: u32 = 1;
const LOWER: u32 = 2;
const AXIS: u32 = 3;
const CHAMFER: u32 = 4;

pub fn build() -> Result<Brep, String> {
    // the ball below bounds the lens above, and the ball above bounds it below
    let below = Region::half_disc(-CENTRE, RADIUS).tagged(&|_, k| [UPPER, AXIS][k]);
    let above = Region::half_disc(CENTRE, RADIUS).tagged(&|_, k| [LOWER, AXIS][k]);
    let lens = boolean(&below, &above, Op::Common, 1e-9 * RADIUS)?;
    let legs = [Leg::FromEdge(ON_UPPER), Leg::FromEdge(ON_LOWER)];
    let cut = chamfer(&lens, &[UPPER], &[LOWER], legs, CHAMFER)?;
    turned(&cut)
}

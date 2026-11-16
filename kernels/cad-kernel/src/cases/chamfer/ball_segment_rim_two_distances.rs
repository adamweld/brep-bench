//! The part of a ball of radius 5 centred on the origin below the plane z = 3: a ball cut flat
//! above its middle, its flat face the disc of radius 4. The flat's rim, where it meets the ball,
//! chamfered with two distances: 2 on the flat face and 1 on the ball, each a straight line from
//! the edge in the plane through the z axis. The chamfer is the cone between the two circles it
//! reaches.
//!
//! The cut ball is a solid of revolution, so it is built as one: its meridian, the ball's half disc
//! in common with the half-plane below z = 3 (a planar Boolean), chamfered at its corner by
//! `corners::chamfer`, whose `Leg::FromEdge` is a straight chord from the corner, and turned once
//! about the z axis.
use super::hemisphere_rim_distance_1_angle_45::turned;
use cad_kernel::corners::{chamfer, Leg};
use cad_kernel::planar::{boolean, Op, Region, Seg, Step};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const LEVEL: f64 = 3.;
const ON_FLAT: f64 = 2.;
const ON_BALL: f64 = 1.;

// the faces of the meridian, by tag (0 is none)
const BALL: u32 = 1;
const FLAT: u32 = 2;
const AXIS: u32 = 3;
const CHAMFER: u32 = 4;

pub fn build() -> Result<Brep, String> {
    let legs = [Leg::FromEdge(ON_FLAT), Leg::FromEdge(ON_BALL)];
    let cut = chamfer(&cut_ball()?, &[FLAT], &[BALL], legs, CHAMFER)?;
    turned(&cut)
}

/// The meridian of the ball below z = 3: its half disc in common with a block whose top is the
/// plane, reaching past the ball everywhere else.
pub fn cut_ball() -> Result<Region, String> {
    let line = |a, b, tag| Step { seg: Seg::Line { a, b }, chart: None, tag };
    let ball = Region::half_disc(0., RADIUS).tagged(&|_, k| [BALL, AXIS][k]);
    let reach = 2. * RADIUS;
    let below = Region::new(vec![vec![
        line([0., -reach], [reach, -reach], 0),
        line([reach, -reach], [reach, LEVEL], 0),
        line([reach, LEVEL], [0., LEVEL], FLAT),
        line([0., LEVEL], [0., -reach], AXIS),
    ]]);
    boolean(&ball, &below, Op::Common, 1e-9 * RADIUS)
}

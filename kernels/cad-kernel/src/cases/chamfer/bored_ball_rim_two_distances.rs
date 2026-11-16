//! A ball of radius 5 centred on the origin, a hole of radius 2 bored right through it along the z
//! axis, meeting the ball in two circles of radius 2 at z = ±√21. The upper one chamfered with two
//! distances: 2 down the bore and 1 on the ball, each a straight line from the edge in the plane
//! through the z axis. The chamfer is the cone between the two circles it reaches; the lower rim
//! stays sharp.
//!
//! The bored ball is a solid of revolution, so it is built as one: its meridian, the ball's half
//! disc less the bore's strip (a planar Boolean), chamfered at its upper corner and turned once
//! about the z axis. Both rims are corners between the ball and the bore, and the kernel's
//! `corners::chamfer` cuts every corner between two faces; so the upper one's flat is taken from
//! `corners::flat`, picked by the corner's point, and the piece it cuts off taken away by a planar
//! Boolean, as `hemisphere_rim_chamfered_to_its_center` does.
use super::hemisphere_rim_chamfered_to_its_center::chamfered_at;
use super::hemisphere_rim_distance_1_angle_45::turned;
use cad_kernel::corners::Leg;
use cad_kernel::planar::{boolean, Op, Region, Seg, Step};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const BORE_RADIUS: f64 = 2.;
const DOWN_BORE: f64 = 2.;
const ON_BALL: f64 = 1.;

// the faces of the meridian, by tag (0 is none)
pub const BALL: u32 = 1;
pub const BORE: u32 = 2;
const CHAMFER: u32 = 3;

pub fn build() -> Result<Brep, String> {
    // the upper rim, at (2, √21), where the bore meets the ball
    let rim = [BORE_RADIUS, (RADIUS * RADIUS - BORE_RADIUS * BORE_RADIUS).sqrt()];
    let legs = [(BORE, Leg::FromEdge(DOWN_BORE)), (BALL, Leg::FromEdge(ON_BALL))];
    let cut = chamfered_at(&bored()?, rim, legs, CHAMFER)?;
    turned(&cut)
}

/// The bored ball's meridian: the ball's half disc less the strip within the bore's radius of the
/// axis, reaching past the ball above and below.
pub fn bored() -> Result<Region, String> {
    let line = |a, b, tag| Step { seg: Seg::Line { a, b }, chart: None, tag };
    let ball = Region::half_disc(0., RADIUS).tagged(&|_, k| [BALL, 0][k]);
    let (r, reach) = (BORE_RADIUS, 2. * RADIUS);
    let bore = Region::new(vec![vec![
        line([-r, -reach], [r, -reach], 0),
        line([r, -reach], [r, reach], BORE),
        line([r, reach], [-r, reach], 0),
        line([-r, reach], [-r, -reach], 0),
    ]]);
    boolean(&ball, &bore, Op::Cut, 1e-9 * RADIUS)
}

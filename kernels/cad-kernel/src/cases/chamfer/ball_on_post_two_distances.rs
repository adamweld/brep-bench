//! A post, the cylinder of radius 3 on the z axis from z = 0 to z = 10, joined to a ball of radius
//! 5 centred at (0, 0, 10). The concave edge where the post's side meets the ball, the circle of
//! radius 3 at z = 6, chamfered with two distances: 2 down the post and 1 on the ball, each a
//! straight line from the edge in the plane through the z axis. A concave chamfer adds material:
//! the cone between the two circles it reaches fills the groove between the post and the ball.
//!
//! The post and the ball are a solid of revolution, so they are built as one: their meridian, the
//! post's rectangle joined to the ball's half disc (a planar Boolean), chamfered at its corner by
//! `corners::chamfer`, whose `Leg::FromEdge` is a straight chord from the corner (a concave corner
//! filled), and turned once about the z axis.
use super::hemisphere_rim_distance_1_angle_45::turned;
use cad_kernel::corners::{chamfer, Leg};
use cad_kernel::planar::{boolean, Op, Region, Seg, Step};
use cad_kernel::topo::Brep;

const POST_RADIUS: f64 = 3.;
const POST_HEIGHT: f64 = 10.;
const RADIUS: f64 = 5.;
const DOWN_POST: f64 = 2.;
const ON_BALL: f64 = 1.;

// the faces of the meridian, by tag (0 is none)
const FLOOR: u32 = 1;
const POST: u32 = 2;
const BALL: u32 = 3;
const AXIS: u32 = 4;
const CHAMFER: u32 = 5;

pub fn build() -> Result<Brep, String> {
    let line = |a, b, tag| Step { seg: Seg::Line { a, b }, chart: None, tag };
    let (r, h) = (POST_RADIUS, POST_HEIGHT);
    let post = Region::new(vec![vec![
        line([0., 0.], [r, 0.], FLOOR),
        line([r, 0.], [r, h], POST),
        line([r, h], [0., h], 0),
        line([0., h], [0., 0.], AXIS),
    ]]);
    // the ball about the post's top
    let ball = Region::half_disc(POST_HEIGHT, RADIUS).tagged(&|_, k| [BALL, AXIS][k]);
    let joined = boolean(&post, &ball, Op::Union, 1e-9 * POST_HEIGHT)?;
    let legs = [Leg::FromEdge(DOWN_POST), Leg::FromEdge(ON_BALL)];
    let cut = chamfer(&joined, &[POST], &[BALL], legs, CHAMFER)?;
    turned(&cut)
}

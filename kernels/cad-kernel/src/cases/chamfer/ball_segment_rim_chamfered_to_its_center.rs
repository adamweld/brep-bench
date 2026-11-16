//! The part of a ball of radius 5 centred on the origin below the plane z = 3, its flat face the
//! disc of radius 4. The flat's rim chamfered with two distances: 4 on the flat face, the whole way
//! to its centre, and 1 on the ball, each a straight line from the edge in the plane through the z
//! axis. The flat shrinks to a point, and the solid is the ball below the circle 1 from the rim
//! under the cone from that circle up to its apex at (0, 0, 3).
//!
//! The cut ball's meridian, as `ball_segment_rim_two_distances` draws it, chamfered at its corner
//! and turned once about the z axis. The chamfer consumes the flat face, which the kernel's
//! `corners::chamfer` declines; so the corner's flat is taken from `corners::flat` and the piece
//! it cuts off taken away by a planar Boolean, as `hemisphere_rim_chamfered_to_its_center` does.
use super::ball_segment_rim_two_distances::cut_ball;
use super::hemisphere_rim_chamfered_to_its_center::chamfered_at;
use super::hemisphere_rim_distance_1_angle_45::turned;
use cad_kernel::corners::Leg;
use cad_kernel::topo::Brep;

const RIM: [f64; 2] = [4., 3.];
const ON_FLAT: f64 = 4.;
const ON_BALL: f64 = 1.;

// the faces of the meridian, by tag, as `cut_ball` tags them
const BALL: u32 = 1;
const FLAT: u32 = 2;
const CHAMFER: u32 = 4;

pub fn build() -> Result<Brep, String> {
    // the rim, the corner at (4, 3), cut by the chord between the flat's centre, 4 from it, and
    // the point 1 from it on the ball
    let legs = [(FLAT, Leg::FromEdge(ON_FLAT)), (BALL, Leg::FromEdge(ON_BALL))];
    let cut = chamfered_at(&cut_ball()?, RIM, legs, CHAMFER)?;
    turned(&cut)
}

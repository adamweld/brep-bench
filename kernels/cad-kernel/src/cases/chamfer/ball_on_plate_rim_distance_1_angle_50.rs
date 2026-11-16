//! A plate, the cylinder of radius 8 on the z axis from z = −6 to z = 0, joined to a ball of radius
//! 5 centred on the origin, whose upper half rises from the plate's top. The concave edge where the
//! ball meets the top, the circle of radius 5, chamfered by a distance and an angle: 1 out from the
//! edge on the plate's top, the chamfer rising from there at 50° to the top back toward the ball.
//! A concave chamfer adds material: it fills the groove between the plate and the ball.
//!
//! The plate and the ball are a solid of revolution, so they are built as one: their meridian, the
//! plate's rectangle joined to the ball's half disc (a planar Boolean), its corner cut by a
//! distance and an angle as `hemisphere_rim_distance_1_angle_45` cuts it (cad-kernel's chamfers
//! take no angle; at a concave corner the cut fills it), and turned once about the z axis.
use super::hemisphere_rim_distance_1_angle_45::{by_distance_and_angle, turned};
use cad_kernel::planar::{boolean, Op, Region, Seg, Step};
use cad_kernel::topo::Brep;

const DISTANCE: f64 = 1.;
const ANGLE: f64 = 50.;

const PLATE_RADIUS: f64 = 8.;
const PLATE_DEPTH: f64 = 6.;
const RADIUS: f64 = 5.;

// the faces of the meridian, by tag (0 is none)
const FLOOR: u32 = 1;
const SIDE: u32 = 2;
const TOP: u32 = 3;
const BALL: u32 = 4;
const AXIS: u32 = 5;
const CHAMFER: u32 = 6;

pub fn build() -> Result<Brep, String> {
    ball_on_plate(DISTANCE, ANGLE)
}

/// The ball on the plate, the edge between them chamfered `distance` out on the plate's top and at
/// `angle` degrees to it.
pub fn ball_on_plate(distance: f64, angle: f64) -> Result<Brep, String> {
    let line = |a, b, tag| Step { seg: Seg::Line { a, b }, chart: None, tag };
    let (r, d) = (PLATE_RADIUS, PLATE_DEPTH);
    let plate = Region::new(vec![vec![
        line([0., -d], [r, -d], FLOOR),
        line([r, -d], [r, 0.], SIDE),
        line([r, 0.], [0., 0.], TOP),
        line([0., 0.], [0., -d], AXIS),
    ]]);
    let ball = Region::half_disc(0., RADIUS).tagged(&|_, k| [BALL, AXIS][k]);
    let joined = boolean(&plate, &ball, Op::Union, 1e-9 * PLATE_RADIUS)?;
    let cut = by_distance_and_angle(&joined, TOP, BALL, distance, angle.to_radians(), CHAMFER)?;
    turned(&cut)
}

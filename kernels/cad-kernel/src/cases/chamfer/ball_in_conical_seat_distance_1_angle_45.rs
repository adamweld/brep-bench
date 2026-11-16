//! A ball of radius 5 about the origin standing in a conical seat: the cylinder of radius 8 about
//! the z axis from z = −6 to z = 5, everything above the cone z = 1 + ρ/2 hollowed out of it,
//! joined to the ball. The seat's cone meets the ball in the circle of radius 4 at z = 3, a concave
//! edge, chamfered by a distance and an angle: 1 up the seat's cone from the edge, the chamfer
//! making 45° with the cone, rising from it back toward the ball. A concave chamfer adds material:
//! it fills the groove between the seat and the ball.
//!
//! The seat and the ball are a solid of revolution, so they are built as one: their meridian, the
//! plate's rectangle less the region above the cone's line, joined to the ball's half disc (planar
//! Booleans), its corner cut by a distance and an angle as `hemisphere_rim_distance_1_angle_45`
//! cuts it (cad-kernel's chamfers take no angle; at a concave corner the cut fills it), and turned
//! once about the z axis.
use super::hemisphere_rim_distance_1_angle_45::{by_distance_and_angle, turned};
use cad_kernel::planar::{boolean, Op, Region, Seg, Step};
use cad_kernel::topo::Brep;

const DISTANCE: f64 = 1.;
const ANGLE: f64 = 45.;

const PLATE_RADIUS: f64 = 8.;
const PLATE_FLOOR: f64 = -6.;
const PLATE_TOP: f64 = 5.;
// the seat's cone, z = APEX + SLOPE·ρ
const APEX: f64 = 1.;
const SLOPE: f64 = 0.5;
const RADIUS: f64 = 5.;

// the faces of the meridian, by tag (0 is none)
const FLOOR: u32 = 1;
const SIDE: u32 = 2;
const SEAT: u32 = 3;
const BALL: u32 = 4;
const AXIS: u32 = 5;
const CHAMFER: u32 = 6;

pub fn build() -> Result<Brep, String> {
    ball_in_seat(DISTANCE, ANGLE)
}

/// The ball in its seat, the edge between them chamfered `distance` up the seat's cone and at
/// `angle` degrees to it.
pub fn ball_in_seat(distance: f64, angle: f64) -> Result<Brep, String> {
    let tol = 1e-9 * PLATE_RADIUS;
    let line = |a, b, tag| Step { seg: Seg::Line { a, b }, chart: None, tag };
    let (r, lo, hi) = (PLATE_RADIUS, PLATE_FLOOR, PLATE_TOP);
    let plate = Region::new(vec![vec![
        line([0., lo], [r, lo], FLOOR),
        line([r, lo], [r, hi], SIDE),
        line([r, hi], [0., hi], 0),
        line([0., hi], [0., lo], AXIS),
    ]]);
    // the hollow: above the cone's line from its apex out past the plate's rim
    let reach = 1.5 * PLATE_RADIUS;
    let out = [reach, APEX + SLOPE * reach];
    let hollow = Region::new(vec![vec![
        line([0., APEX], out, SEAT),
        line(out, [0., out[1]], 0),
        line([0., out[1]], [0., APEX], 0),
    ]]);
    let seat = boolean(&plate, &hollow, Op::Cut, tol)?;
    let ball = Region::half_disc(0., RADIUS).tagged(&|_, k| [BALL, AXIS][k]);
    let joined = boolean(&seat, &ball, Op::Union, tol)?;
    let cut = by_distance_and_angle(&joined, SEAT, BALL, distance, angle.to_radians(), CHAMFER)?;
    turned(&cut)
}

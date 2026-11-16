//! A frustum on the z axis, widening upward from the disc of radius 2 at z = 0 to radius 4 at
//! z = 3, capped by the part of the ball of radius 5 about the origin above z = 3. Its rim, where
//! the ball meets the frustum's cone, chamfered by a distance and an angle: 1 down the cone from
//! the rim, the chamfer making 45° with the cone's side, turned toward the z axis.
//!
//! The capped frustum is a solid of revolution, so it is built as one: its meridian, the frustum's
//! trapezoid joined to the ball's half disc kept above z = 3 (planar Booleans), its corner cut by a
//! distance and an angle as `hemisphere_rim_distance_1_angle_45` cuts it (cad-kernel's chamfers
//! take no angle), and turned once about the z axis.
use super::hemisphere_rim_distance_1_angle_45::{by_distance_and_angle, turned};
use cad_kernel::planar::{boolean, Op, Region, Seg, Step};
use cad_kernel::topo::Brep;

const DISTANCE: f64 = 1.;
const ANGLE: f64 = 45.;

const FOOT: f64 = 2.;
const RIM: f64 = 4.;
const HEIGHT: f64 = 3.;
const RADIUS: f64 = 5.;

// the faces of the meridian, by tag (0 is none)
const FLOOR: u32 = 1;
const CONE: u32 = 2;
const BALL: u32 = 3;
const AXIS: u32 = 4;
const CHAMFER: u32 = 5;

pub fn build() -> Result<Brep, String> {
    capped_frustum(DISTANCE, ANGLE)
}

/// The capped frustum, its rim chamfered `distance` down the cone and at `angle` degrees to it.
pub fn capped_frustum(distance: f64, angle: f64) -> Result<Brep, String> {
    let tol = 1e-9 * RADIUS;
    let line = |a, b, tag| Step { seg: Seg::Line { a, b }, chart: None, tag };
    let frustum = Region::new(vec![vec![
        line([0., 0.], [FOOT, 0.], FLOOR),
        line([FOOT, 0.], [RIM, HEIGHT], CONE),
        line([RIM, HEIGHT], [0., HEIGHT], 0),
        line([0., HEIGHT], [0., 0.], AXIS),
    ]]);
    // the cap: the ball's half disc in common with a block standing on z = 3, past the ball
    let ball = Region::half_disc(0., RADIUS).tagged(&|_, k| [BALL, AXIS][k]);
    let reach = 2. * RADIUS;
    let above = Region::new(vec![vec![
        line([0., HEIGHT], [reach, HEIGHT], 0),
        line([reach, HEIGHT], [reach, reach], 0),
        line([reach, reach], [0., reach], 0),
        line([0., reach], [0., HEIGHT], AXIS),
    ]]);
    let cap = boolean(&ball, &above, Op::Common, tol)?;
    let whole = boolean(&frustum, &cap, Op::Union, tol)?;
    let cut = by_distance_and_angle(&whole, CONE, BALL, distance, angle.to_radians(), CHAMFER)?;
    turned(&cut)
}

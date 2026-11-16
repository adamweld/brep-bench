//! A spherical sector: the ball of radius 5 centred on the origin inside the cone from the origin
//! about the z axis, opening upward with half angle α, cos α = 3/5. Its rim, where the cone meets
//! the ball, chamfered by a distance and an angle: 1 down the cone from the rim, the chamfer making
//! 45° with the cone's side, turned toward the z axis into the sector.
//!
//! The sector is a solid of revolution, so it is built as one: its meridian, the half disc of the
//! ball in common with the cone's wedge (a planar Boolean), its corner cut by a distance and an
//! angle as `hemisphere_rim_distance_1_angle_45` cuts it (cad-kernel's chamfers take no angle), and
//! turned once about the z axis.
use super::hemisphere_rim_distance_1_angle_45::{by_distance_and_angle, turned};
use cad_kernel::planar::{boolean, Op, Region, Seg, Step};
use cad_kernel::topo::Brep;

const COS_ALPHA: f64 = 3. / 5.;
const DISTANCE: f64 = 1.;
const ANGLE: f64 = 45.;

const RADIUS: f64 = 5.;

// the faces of the meridian, by tag (0 is none)
pub const BALL: u32 = 1;
pub const SIDE: u32 = 2;
const AXIS: u32 = 3;
const CHAMFER: u32 = 4;

pub fn build() -> Result<Brep, String> {
    sector(COS_ALPHA, DISTANCE, ANGLE)
}

/// The sector of half angle α (by its cosine), its rim chamfered `distance` down the cone and at
/// `angle` degrees to it.
pub fn sector(cos_alpha: f64, distance: f64, angle: f64) -> Result<Brep, String> {
    let cut = by_distance_and_angle(&meridian(cos_alpha)?, SIDE, BALL, distance, angle.to_radians(),
                                    CHAMFER)?;
    turned(&cut)
}

/// The sector's meridian in the (ρ, z) half-plane: the ball's half disc in common with the cone's
/// wedge, the triangle between the axis and the cone's side out past the ball.
pub fn meridian(cos_alpha: f64) -> Result<Region, String> {
    let step = |seg, tag| Step { seg, chart: None, tag };
    let line = |a, b, tag| step(Seg::Line { a, b }, tag);
    let ball = Region::half_disc(0., RADIUS).tagged(&|_, k| [BALL, AXIS][k]);
    // the cone's side from the origin along (sin α, cos α), twice the ball's radius
    let reach = 2. * RADIUS;
    let out = [reach * (1. - cos_alpha * cos_alpha).sqrt(), reach * cos_alpha];
    let wedge = Region::new(vec![vec![
        line([0., 0.], out, SIDE),
        line(out, [0., reach], 0),
        line([0., reach], [0., 0.], AXIS),
    ]]);
    boolean(&ball, &wedge, Op::Common, 1e-9 * RADIUS)
}

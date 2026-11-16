//! The cone on z with its base of radius 6 at z = 0 and its apex at z = 12, its tip rounded by a
//! ball of radius 1 tangent to its side, shelled inward to a wall 1.5 thick, open at its base:
//! thicker than the tip's radius, so the cavity is a plain cone, its apex where the moved side
//! closes, standing on the base's plane.
use crate::sketch::faces_through;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, revolve, Profile, ProfileEdge};
use cad_kernel::geom::Frame;
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, TAU};

const BASE_RADIUS: f64 = 6.;
const APEX: f64 = 12.;
const TIP: f64 = 1.;
const THICKNESS: f64 = 1.5;

/// The cone with its tip rounded: its half-section in the plane y = 0, the base, the side up to
/// where the ball touches it, and the ball's arc on to the axis, turned about z.
fn ball_tipped() -> Result<Brep, String> {
    // the side's half-angle α, its lean 6 in 12; the ball's centre on the axis 1 / sin α below
    // the apex, and the side touched where the ball's radius square to it, at α above the level,
    // meets it
    let alpha = (BASE_RADIUS / APEX).atan();
    let centre = APEX - TIP / alpha.sin();
    let touch = [TIP * alpha.cos(), 0., centre + TIP * alpha.sin()];
    let ball = Frame::new([0., 0., centre], [0., -1., 0.], [1., 0., 0.]);
    let edges = vec![
        ProfileEdge::Line { a: [0., 0., 0.], b: [BASE_RADIUS, 0., 0.] },
        ProfileEdge::Line { a: [BASE_RADIUS, 0., 0.], b: touch },
        ProfileEdge::Arc { frame: ball, r: TIP, span: Some([alpha, FRAC_PI_2]) },
        ProfileEdge::Line { a: [0., 0., centre + TIP], b: [0., 0., 0.] },
    ];
    let section = Profile { origin: [0.; 3], normal: [0., -1., 0.], loops: vec![edges],
                            names: Vec::new() };
    revolve(&section, [0.; 3], [0., 0., 1.], TAU)
}

pub fn build() -> Result<Brep, String> {
    // cad-kernel shells a solid as the points within the wall of its faces, each face swept
    // inward along its normal, and refuses the tip's sphere swept past its centre ("no point
    // stands that far off it"), as a ball of radius 1 is by a wall 1.5 thick; and it has no round
    // of a vertex. So the sharp cone is shelled first, and its tip rounded after, as a modeller
    // orders the two features when the shell fails, by keeping it within the ball-tipped cone:
    // the round takes only material within the wall over the tip, so the cavity is the same
    // either way.
    let sharp = cone([0., 0., 0.], [0., 0., 1.], BASE_RADIUS, 0., APEX)?;
    let open = faces_through(&sharp, &[[0., 0., 0.]])?;
    let hollow = shelled(&sharp, &open, THICKNESS, Side::In, Join::Round)?;
    Ok(combined(&hollow, &ball_tipped()?, Op::Common, 0.)?)
}

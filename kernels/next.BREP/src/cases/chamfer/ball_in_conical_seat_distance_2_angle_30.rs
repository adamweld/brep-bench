//! A ball in a conical seat, the rim between them chamfered by a distance of 2 and an angle of 30°.
//! A ball of radius 5 about the origin joined to a plate (radius 8, z = −6 to 5) hollowed above the
//! cone z = 1 + r/2; the concave circle where the cone meets the ball, (4, 3) in (ρ, z), chamfered by
//! a distance and an angle: 2 up the cone from the edge, the chamfer leaving the cone at 30° back
//! toward the ball, filling the groove.
//!
//! The body is its section turned a whole turn about the z axis: the plate's bottom and side, the
//! cone down from (8, 5) to the edge, the ball's arc up to its pole, and the axis back down.
use crate::solids::fillet::{faces_of_edge_at, is_ball_about};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{chamfer_edges_angle, make_arc, make_line, revolve_profile_brep, BrepSolid};
use std::f64::consts::{FRAC_PI_2, TAU};

const UP_CONE: f64 = 2.0;
const ANGLE_DEGREES: f64 = 30.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, x, z) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let at = |rho: f64, height: f64| p(rho, 0.0, height);
    let section = [
        make_line(at(0.0, -6.0), at(8.0, -6.0))?,
        make_line(at(8.0, -6.0), at(8.0, 5.0))?,
        make_line(at(8.0, 5.0), at(4.0, 3.0))?,
        make_arc(o, x, z, 5.0, 3f64.atan2(4.0), FRAC_PI_2)?,
        make_line(at(0.0, 5.0), at(0.0, -6.0))?,
    ];
    let body = revolve_profile_brep(&section, o, z, TAU)?;
    // The edge, by a point on it. The kernel's distance-angle chamfer measures its distance on the
    // edge's first face, in the order of the solid's shells, and its angle from that face: the
    // seat's cone here. A body that lists the ball first is refused rather than chamfered from the
    // wrong side.
    let edge = p(0.0, -4.0, 3.0);
    match faces_of_edge_at(&body, edge).first() {
        Some(face) if !is_ball_about(face, o) => {}
        Some(_) => {
            return Err(Refusal::from(
                "the kernel's distance-angle chamfer measures from the edge's first face, which is \
                 the ball here, and cannot be told to measure from the seat",
            ))
        }
        None => return Err(Refusal::from(format!("no edge through {edge:?}"))),
    }
    Ok(chamfer_edges_angle(&body, &[edge], None, UP_CONE, ANGLE_DEGREES.to_radians(), None)?)
}

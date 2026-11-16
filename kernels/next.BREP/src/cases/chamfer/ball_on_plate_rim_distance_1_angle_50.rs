//! A plate (the cylinder of radius 8 from z = −6 to 0) with a ball of radius 5 about the origin
//! standing in it, the concave circle where the ball meets the plate's top chamfered by a distance
//! and an angle: 1 out from the edge on the plate, the chamfer rising at 50° from the plate back to
//! the ball, filling the groove.
//!
//! The body is its section turned a whole turn about the z axis: the plate's bottom, side and
//! top in to the edge at (5, 0, 0), the ball's quarter circle up to its pole, and the axis back
//! down.
use crate::solids::fillet::{faces_of_edge_at, is_ball_about};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{chamfer_edges_angle, make_arc, make_line, revolve_profile_brep, BrepSolid};
use std::f64::consts::{FRAC_PI_2, TAU};

const ON_PLATE: f64 = 1.0;
const ANGLE_DEGREES: f64 = 50.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, x, z) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let at = |rho: f64, height: f64| p(rho, 0.0, height);
    let section = [
        make_line(at(0.0, -6.0), at(8.0, -6.0))?,
        make_line(at(8.0, -6.0), at(8.0, 0.0))?,
        make_line(at(8.0, 0.0), at(5.0, 0.0))?,
        make_arc(o, x, z, 5.0, 0.0, FRAC_PI_2)?,
        make_line(at(0.0, 5.0), at(0.0, -6.0))?,
    ];
    let body = revolve_profile_brep(&section, o, z, TAU)?;
    // The edge, by a point on it. The kernel's distance-angle chamfer measures its distance on the
    // edge's first face, in the order of the solid's shells, and its angle from that face: the
    // plate's top here. It has no way to name the other face, so a body that lists the ball first
    // is refused rather than chamfered from the wrong side.
    let edge = p(0.0, -5.0, 0.0);
    match faces_of_edge_at(&body, edge).first() {
        Some(face) if !is_ball_about(face, o) => {}
        Some(_) => {
            return Err(Refusal::from(
                "the kernel's distance-angle chamfer measures from the edge's first face, which is \
                 the ball here, and cannot be told to measure from the plate",
            ))
        }
        None => return Err(Refusal::from(format!("no edge through {edge:?}"))),
    }
    Ok(chamfer_edges_angle(&body, &[edge], None, ON_PLATE, ANGLE_DEGREES.to_radians(), None)?)
}

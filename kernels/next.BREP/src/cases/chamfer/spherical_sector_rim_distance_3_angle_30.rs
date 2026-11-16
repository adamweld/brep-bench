//! The spherical sector of the ball of radius 5 about the origin inside the cone with its apex at
//! the origin about the z axis, cos α = 3/5, its rim (the circle of radius 4 at z = 3) chamfered by
//! a distance and an angle: 3 down the cone from the rim, the chamfer leaving the cone's side at
//! 30° toward the ball.
//!
//! The sector is its section turned a whole turn about the z axis: the line from the origin to
//! (4, 0, 3), the arc of radius 5 from there to the pole (0, 0, 5), and the axis back down.
use crate::solids::fillet::{faces_of_edge_at, is_ball_about};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{chamfer_edges_angle, make_arc, make_line, revolve_profile_brep, BrepSolid};
use std::f64::consts::{FRAC_PI_2, TAU};

const DOWN_CONE: f64 = 3.0;
const ANGLE_DEGREES: f64 = 30.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, x, z) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let section = [
        make_line(o, p(4.0, 0.0, 3.0))?,
        make_arc(o, x, z, 5.0, 3f64.atan2(4.0), FRAC_PI_2)?,
        make_line(p(0.0, 0.0, 5.0), o)?,
    ];
    let sector = revolve_profile_brep(&section, o, z, TAU)?;
    // The rim, by a point on it. The kernel's distance-angle chamfer measures its distance on the
    // edge's first face, in the order of the solid's shells, and its angle from that face: the
    // cone here. It has no way to name the other face, so a body that lists the ball first is
    // refused rather than chamfered from the wrong side.
    let rim = p(0.0, -4.0, 3.0);
    match faces_of_edge_at(&sector, rim).first() {
        Some(face) if !is_ball_about(face, o) => {}
        Some(_) => {
            return Err(Refusal::from(
                "the kernel's distance-angle chamfer measures from the edge's first face, which is \
                 the ball here, and cannot be told to measure from the cone",
            ))
        }
        None => return Err(Refusal::from(format!("no edge through {rim:?}"))),
    }
    Ok(chamfer_edges_angle(&sector, &[rim], None, DOWN_CONE, ANGLE_DEGREES.to_radians(), None)?)
}

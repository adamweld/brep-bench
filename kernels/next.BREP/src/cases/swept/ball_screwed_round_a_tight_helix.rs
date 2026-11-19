//! The solid a ball of radius 2 fills as its centre runs two turns of the helix of radius 1.5 about
//! the z axis rising 3 a turn: everything within 2 of the helix's arc.
//!
//! The helix turns tighter (radius of curvature ρ = (1.5² + p²)/1.5 ≈ 1.652, p = 3/2π) than the
//! ball, so the tube about it folds. Elementary geometry instead. Take a point x within 2 of the
//! arc and its nearest point c(t) there. Either t is an end (x is in the ball at that end), or x lies
//! in the plane square to the helix at c(t) (first order), with (x − c)·N ≤ ρ along the principal
//! normal N, toward the axis (second order: 1 − (x − c)·N/ρ ≥ 0). So the solid is the union of the
//! discs of radius 2 square to the helix, each cut by the chord (x − c)·N = ρ, and the balls at both
//! ends. Cut at h = 1.6 instead, short of ρ, the discs sweep without folding; the points they then
//! leave out, 1.6 < (x − c)·N ≤ ρ, lie within 0.45 of the axis (|x − c| ≤ 2 and the plane's tilt), and
//! every point within 0.5 of the axis from z = 0 to 6 is within 2 of the arc (the helix's point at
//! its height is within 1.5 + 0.5). A cylinder of radius 0.5 there covers them, but for a sliver
//! near each end, just past z = 0 and 6, that the cut discs and the balls do not reach: sampled, one
//! band point in 400000; built, it moves the volume by 5e-9 and the area by 8e-7 of the solid's.
//! Each eighth of the two turns of cut discs is lofted through their exact sections at 33 points of
//! it; the eight lofts, the balls and the cylinder are united.
use crate::solids::p;
use crate::solids::swept::{planar, Piece};
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, loft_profile_brep, make_cylinder_brep, make_sphere_brep, BooleanOperation, BrepSolid, NurbsCurve};
use std::f64::consts::{PI, TAU};

const RADIUS: f64 = 1.5;
const BALL: f64 = 2.0;
const CUT: f64 = 1.6;

pub fn build() -> Result<BrepSolid, Refusal> {
    let rise = 3.0 / TAU;
    let speed = (RADIUS * RADIUS + rise * rise).sqrt();
    let centre = |t: f64| p(RADIUS * t.cos(), RADIUS * t.sin(), rise * t);
    // The chord's ends on the disc, in (along N, along B), and the arc's angle there.
    let half = (BALL * BALL - CUT * CUT).sqrt();
    let corner = half.atan2(CUT);
    let section = |t: f64| -> Result<Vec<NurbsCurve>, Refusal> {
        let tangent = p(-RADIUS * t.sin(), RADIUS * t.cos(), rise).scale(1.0 / speed);
        let normal = p(-t.cos(), -t.sin(), 0.0);
        Ok(planar(
            &[
                Piece::Arc { centre: [0.0, 0.0], radius: BALL, start: corner, end: TAU - corner },
                Piece::Line { from: [CUT, -half], to: [CUT, half] },
            ],
            centre(t),
            normal,
            tangent.cross(normal),
        )?)
    };
    let mut parts = Vec::new();
    for k in 0..8 {
        let sections: Vec<_> = (0..33).map(|i| section(PI / 2.0 * (k as f64 + i as f64 / 32.0))).collect::<Result<_, _>>()?;
        parts.push(loft_profile_brep(&sections)?);
    }
    let up = p(0.0, 0.0, 1.0);
    parts.push(make_sphere_brep(centre(0.0), BALL, up)?);
    parts.push(make_sphere_brep(centre(2.0 * TAU), BALL, up)?);
    parts.push(make_cylinder_brep(p(0.0, 0.0, 0.0), up, 0.5, 6.0)?);
    Ok(boolean_operation_nary(&parts, BooleanOperation::Union)?)
}

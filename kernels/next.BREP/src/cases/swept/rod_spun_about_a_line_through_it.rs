//! Everything a rod passes through as it turns rigidly about the z axis by 120°, right-handed: the
//! solid cylinder of radius 0.5 about the x axis from x = −3 to x = 5, its ends flat.
//!
//! next.BREP's sweep of a solid builds a turn only where both its ends stay clear of the region
//! asked for, and here the result is the whole sweep. By elementary geometry, arm by arm: the rod
//! is its arm x ≥ 0 and its arm x ≤ 0, and sweeps what they sweep. At height z (|z| ≤ 0.5,
//! w = √(0.25 − z²)) the arm x ≥ 0 is the rectangle 0 ≤ x ≤ 5, |y| ≤ w, which a circle of radius ρ
//! about the z axis meets in one arc about angle 0 holding it while ρ ≤ 5, and beyond, out to
//! √(25 + w²), in two arcs ±[a, b] with a ≤ acos(5/√25.25) < 6°. Turned by 120° > 2a, the two arcs
//! sweep the same angles as the arc [−b, b] they leave a gap in, which is the arm with the gap
//! filled: its points beyond x = 5 within √(25.25) of the origin, a cap of that ball (whose points
//! all lie within 0.5 of the x axis). The filled arm meets every circle in one arc holding angle 0,
//! so it sweeps itself where it starts and where it stops and its section by the half plane at
//! angle 0, ρ ≤ √(25.25 − z²), |z| ≤ 0.5, turned between them. The arm x ≤ 0 likewise, about
//! angle 180°, with the cap of the ball of radius √9.25 beyond x = −3 and a gap under 10°. The
//! rod with both caps, where it starts and where it stops, and the two sections turned by the
//! kernel's revolve, are united.
use crate::solids::p;
use crate::solids::sweep::{moved, turn_z};
use crate::solids::swept::{planar, Piece};
use crate::Refusal;
use brep_kernel::{
    boolean_operation, boolean_operation_nary, make_box_brep, make_cylinder_brep, make_sphere_brep,
    revolve_profile_brep, BooleanOperation, BooleanOptions, BrepSolid,
};
use std::f64::consts::PI;

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let origin = p(0.0, 0.0, 0.0);
    let turn = 120f64.to_radians();
    let rod = make_cylinder_brep(p(-3.0, 0.0, 0.0), x, 0.5, 8.0)?;
    // the cap of the ball of radius √(end² + 0.25) about the origin beyond the end x = `end`
    let cap = |end: f64| -> Result<BrepSolid, Refusal> {
        let ball = make_sphere_brep(origin, (end * end + 0.25f64).sqrt(), z)?;
        let beyond = make_box_brep(p(if end > 0.0 { end } else { end - 1.0 }, -1.0, -1.0), 1.0, 2.0, 2.0)?;
        Ok(boolean_operation(&ball, &beyond, BooleanOperation::Intersect, &BooleanOptions::default())?)
    };
    let filled = boolean_operation_nary(&[rod, cap(5.0)?, cap(-3.0)?], BooleanOperation::Union)?;
    // an arm's section by the half plane at angle 0, out to the ball of radius √(length² + 0.25)
    let section = |length: f64| -> Result<BrepSolid, Refusal> {
        let radius = (length * length + 0.25f64).sqrt();
        let half = (0.5 / radius).asin();
        let profile = planar(&[
            Piece::Line { from: [0.0, -0.5], to: [length, -0.5] },
            Piece::Arc { centre: [0.0, 0.0], radius, start: -half, end: half },
            Piece::Line { from: [length, 0.5], to: [0.0, 0.5] },
            Piece::Line { from: [0.0, 0.5], to: [0.0, -0.5] },
        ], origin, x, z)?;
        Ok(revolve_profile_brep(&profile, origin, z, turn)?)
    };
    let parts = vec![
        filled.clone(),
        moved(&filled, turn_z(turn, origin)?)?,
        section(5.0)?,
        moved(&section(3.0)?, turn_z(PI, origin)?)?,
    ];
    Ok(boolean_operation_nary(&parts, BooleanOperation::Union)?)
}

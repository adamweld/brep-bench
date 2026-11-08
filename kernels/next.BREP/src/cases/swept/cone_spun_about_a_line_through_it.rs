//! The solid a cone fills as it is spun 150° about the z axis, which crosses its axis inside it.
//!
//! Through the kernel's sweep of a solid (`sweep_solid_within`): the cone is a solid of revolution
//! about the line through the origin along w = (cos 30°, 0, sin 30°), its meridian the triangle
//! from its point at −2w out to its base of radius 7 tan 25° at +5w; turned about the z axis from
//! 0° to 150°, asked for within 10 of the z axis from z = −6 to 6, which holds the whole sweep. The
//! kernel's sweep takes a turn only where neither end position reaches the region asked for, so it
//! may refuse.
use crate::solids::p;
use crate::Refusal;
#[allow(unused_imports)]
use brep_kernel::{
    boolean_operation_nary, make_box_brep, make_cylinder_brep, revolve_profile_brep,
    sweep_solid_within, BooleanOperation, BrepSolid, OutlinePiece, ScrewStep, SolidMotion,
    SweepClip, SweptTool, Vec3,
};
#[allow(unused_imports)]
use std::f64::consts::{FRAC_PI_2, PI, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut bodies = Vec::new();
    let tool0 = SweptTool::Revolved {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(30f64.to_radians().cos(), 0.0, 30f64.to_radians().sin()),
        radial: p(-(30f64.to_radians().sin()), 0.0, 30f64.to_radians().cos()),
        meridian: { let base = 7.0 * 25f64.to_radians().tan(); vec![
            OutlinePiece::Line { from: [0.0, -2.0], to: [base, 5.0] },
            OutlinePiece::Line { from: [base, 5.0], to: [0.0, 5.0] },
            OutlinePiece::Line { from: [0.0, 5.0], to: [0.0, -2.0] },
        ] },
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 0.0 },
        ],
        from: 0.0,
        to: 150f64.to_radians(),
    };
    let clip0 = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 10.0],
        axial: [-6.0, 6.0],
        angle: None,
    };
    bodies.extend(sweep_solid_within(&tool0, &motion0, &clip0)?);
    Ok(boolean_operation_nary(&bodies, BooleanOperation::Union)?)
}

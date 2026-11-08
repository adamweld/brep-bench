//! The solid a thick disc fills as it is turned 90° about the z axis, which passes through its rim.
//!
//! Through the kernel's sweep of a solid (`sweep_solid_within`): the disc is a solid of revolution
//! about the line through (2, 0, 0) along y, its meridian the rectangle out to radius 2 from y = 0
//! to y = 0.5; turned about the z axis from 0° to 90°, asked for within 5 of the z axis from z =
//! −2.5 to 2.5, which holds the whole sweep. The kernel's sweep takes a turn only where neither end
//! position reaches the region asked for, so it may refuse.
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
        axis_origin: p(2.0, 0.0, 0.0),
        axis_direction: p(0.0, 1.0, 0.0),
        radial: p(1.0, 0.0, 0.0),
        meridian: vec![
            OutlinePiece::Line { from: [0.0, 0.0], to: [2.0, 0.0] },
            OutlinePiece::Line { from: [2.0, 0.0], to: [2.0, 0.5] },
            OutlinePiece::Line { from: [2.0, 0.5], to: [0.0, 0.5] },
            OutlinePiece::Line { from: [0.0, 0.5], to: [0.0, 0.0] },
        ],
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 0.0 },
        ],
        from: 0.0,
        to: FRAC_PI_2,
    };
    let clip0 = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 5.0],
        axial: [-2.5, 2.5],
        angle: None,
    };
    bodies.extend(sweep_solid_within(&tool0, &motion0, &clip0)?);
    Ok(boolean_operation_nary(&bodies, BooleanOperation::Union)?)
}

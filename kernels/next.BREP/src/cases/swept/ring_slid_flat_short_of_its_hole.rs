//! The solid a ring fills as it slides 6 along x, square to its axis, less than its hole's width.
//!
//! Through the kernel's sweep of a solid (`sweep_solid_within`): the ring is a solid of revolution
//! about the z axis, its meridian the circle of radius 2 at 6 from the axis; slid 6 along x, asked
//! for within 12 of the vertical line through (3, 0) from z = −2.5 to 2.5, which holds the whole
//! sweep. The kernel's sweep takes a solid of revolution only along a screw or turn whose end
//! positions stay out of the region asked for, so it may refuse.
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
        axis_direction: p(0.0, 0.0, 1.0),
        radial: p(1.0, 0.0, 0.0),
        meridian: vec![OutlinePiece::Arc { centre: [6.0, 0.0], radius: 2.0, start: 0.0, end: TAU }],
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(1.0, 0.0, 0.0), turn: 0.0, advance: 6.0 },
        ],
        from: 0.0,
        to: 1.0,
    };
    let clip0 = SweepClip {
        axis_origin: p(3.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 12.0],
        axial: [-2.5, 2.5],
        angle: None,
    };
    bodies.extend(sweep_solid_within(&tool0, &motion0, &clip0)?);
    Ok(boolean_operation_nary(&bodies, BooleanOperation::Union)?)
}

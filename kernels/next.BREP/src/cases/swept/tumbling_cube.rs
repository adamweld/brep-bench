//! The solid a cube fills as it tumbles: turned by t/4 about the vertical line through (2, 0, 0),
//! then by −t about the line through (0, 0.5, 0.5) along x, for t from −75° to 75°.
//!
//! Through the kernel's sweep of a solid (`sweep_solid_within`), asked for the cube as a prism (its
//! square 2.4 ≤ x ≤ 3.6, |y| ≤ 0.6 from z = 1.4 to 2.6) along those two turns, within 10 of the z
//! axis from z = −10 to 10, which holds the whole sweep. The kernel's sweep takes a prism only
//! along a planar motion, and two turns about skew axes are none, so it may refuse.
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
    let tool0 = SweptTool::Prism {
        origin: p(0.0, 0.0, 0.0),
        normal: p(0.0, 0.0, 1.0),
        x_axis: p(1.0, 0.0, 0.0),
        outline: vec![
            OutlinePiece::Line { from: [2.4, -0.6], to: [3.6, -0.6] },
            OutlinePiece::Line { from: [3.6, -0.6], to: [3.6, 0.6] },
            OutlinePiece::Line { from: [3.6, 0.6], to: [2.4, 0.6] },
            OutlinePiece::Line { from: [2.4, 0.6], to: [2.4, -0.6] },
        ],
        from: 1.4,
        to: 2.6,
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(2.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 0.25, advance: 0.0 },
            ScrewStep { axis_origin: p(0.0, 0.5, 0.5), axis_direction: p(1.0, 0.0, 0.0), turn: -1.0, advance: 0.0 },
        ],
        from: (-75f64).to_radians(),
        to: 75f64.to_radians(),
    };
    let clip0 = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 10.0],
        axial: [-10.0, 10.0],
        angle: None,
    };
    bodies.extend(sweep_solid_within(&tool0, &motion0, &clip0)?);
    Ok(boolean_operation_nary(&bodies, BooleanOperation::Union)?)
}

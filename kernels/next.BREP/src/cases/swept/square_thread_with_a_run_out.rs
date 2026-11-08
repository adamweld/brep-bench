//! A rod threaded partway up by a bar that is then withdrawn straight out of its groove.
//!
//! What a two-piece motion sweeps is the union of what each piece sweeps. Through the kernel's
//! sweep of a solid (`sweep_solid_within`): the bar as a prism (its rectangle 4 ≤ x ≤ 12, |y| ≤ 8
//! from z = −1 to 1) along the screw about the z axis that rises 6 a turn from −90° to +900°; and
//! the bar where that stops (turned 900°, raised 15: the rectangle −12 ≤ x ≤ −4, |y| ≤ 8 from z =
//! 14 to 16) slid 3 along −x, a planar motion. Each is asked for within 5.2 of the z axis from z =
//! −0.4 to 20.4 (the slide only over the bar's own height), and both are cut from the rod.
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
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let mut operands = vec![rod];
    let tool0 = SweptTool::Prism {
        origin: p(0.0, 0.0, 0.0),
        normal: p(0.0, 0.0, 1.0),
        x_axis: p(1.0, 0.0, 0.0),
        outline: vec![
            OutlinePiece::Line { from: [4.0, -8.0], to: [12.0, -8.0] },
            OutlinePiece::Line { from: [12.0, -8.0], to: [12.0, 8.0] },
            OutlinePiece::Line { from: [12.0, 8.0], to: [4.0, 8.0] },
            OutlinePiece::Line { from: [4.0, 8.0], to: [4.0, -8.0] },
        ],
        from: -1.0,
        to: 1.0,
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 6.0 / TAU },
        ],
        from: (-90.0f64).to_radians(),
        to: 900.0f64.to_radians(),
    };
    let clip0 = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 5.2],
        axial: [-0.4, 20.4],
        angle: None,
    };
    operands.extend(sweep_solid_within(&tool0, &motion0, &clip0)?);
    let tool1 = SweptTool::Prism {
        origin: p(0.0, 0.0, 0.0),
        normal: p(0.0, 0.0, 1.0),
        x_axis: p(1.0, 0.0, 0.0),
        outline: vec![
            OutlinePiece::Line { from: [-12.0, -8.0], to: [-4.0, -8.0] },
            OutlinePiece::Line { from: [-4.0, -8.0], to: [-4.0, 8.0] },
            OutlinePiece::Line { from: [-4.0, 8.0], to: [-12.0, 8.0] },
            OutlinePiece::Line { from: [-12.0, 8.0], to: [-12.0, -8.0] },
        ],
        from: 14.0,
        to: 16.0,
    };
    let motion1 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(1.0, 0.0, 0.0), turn: 0.0, advance: -3.0 },
        ],
        from: 0.0,
        to: 1.0,
    };
    let clip1 = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 5.2],
        axial: [14.0, 16.0],
        angle: None,
    };
    operands.extend(sweep_solid_within(&tool1, &motion1, &clip1)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

//! A ring sector notched by a ball on a leaning stem swung partway into it.
//!
//! What a union sweeps is the union of what its parts sweep. Through the kernel's sweep of a solid
//! (`sweep_solid_within`): the ball, a solid of revolution about the vertical line through (10, 0,
//! 0) whose meridian is the half disc of radius 0.8, and the stem, one about the line through (10,
//! 0, 0) along (0, sin 20°, cos 20°) whose meridian is the rectangle out to radius 0.4 from 0 to 3
//! along it; each turned about the z axis from −140° to +30°, asked for between 7.5 and 12.5 from
//! the z axis, from z = −1.5 to 1.5 and from −10° to 70°; cut from the block (the rectangle 8 ≤ x ≤
//! 12, |z| ≤ 1 revolved from 0° to 60°). The kernel's sweep takes a turn only where neither end
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
    let section = [p(8.0, 0.0, -1.0), p(12.0, 0.0, -1.0), p(12.0, 0.0, 1.0), p(8.0, 0.0, 1.0)];
    let block = revolve_profile_brep(&crate::solids::polygon(&section)?, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 60f64.to_radians())?;
    let mut operands = vec![block];
    let tool0 = SweptTool::Revolved {
        axis_origin: p(10.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        radial: p(1.0, 0.0, 0.0),
        meridian: vec![
            OutlinePiece::Arc { centre: [0.0, 0.0], radius: 0.8, start: -FRAC_PI_2, end: FRAC_PI_2 },
            OutlinePiece::Line { from: [0.0, 0.8], to: [0.0, -0.8] },
        ],
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 0.0 },
        ],
        from: (-140f64).to_radians(),
        to: 30f64.to_radians(),
    };
    let clip0 = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [7.5, 12.5],
        axial: [-1.5, 1.5],
        angle: Some([(-10f64).to_radians(), 70f64.to_radians()]),
    };
    operands.extend(sweep_solid_within(&tool0, &motion0, &clip0)?);
    let tool1 = SweptTool::Revolved {
        axis_origin: p(10.0, 0.0, 0.0),
        axis_direction: p(0.0, 20f64.to_radians().sin(), 20f64.to_radians().cos()),
        radial: p(1.0, 0.0, 0.0),
        meridian: vec![
            OutlinePiece::Line { from: [0.0, 0.0], to: [0.4, 0.0] },
            OutlinePiece::Line { from: [0.4, 0.0], to: [0.4, 3.0] },
            OutlinePiece::Line { from: [0.4, 3.0], to: [0.0, 3.0] },
            OutlinePiece::Line { from: [0.0, 3.0], to: [0.0, 0.0] },
        ],
    };
    let motion1 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 0.0 },
        ],
        from: (-140f64).to_radians(),
        to: 30f64.to_radians(),
    };
    let clip1 = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [7.5, 12.5],
        axial: [-1.5, 1.5],
        angle: Some([(-10f64).to_radians(), 70f64.to_radians()]),
    };
    operands.extend(sweep_solid_within(&tool1, &motion1, &clip1)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

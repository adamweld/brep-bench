//! A ring sector notched by a drum swung partway into it.
//!
//! The kernel's sweep of a solid (`sweep_solid_within`, its planar motion of a prism, which
//! includes the end positions exactly): the drum is the circle of radius 1 about (10, 0) from z =
//! −0.5 to 0.5, a prism along the z axis, turned about the z axis from −140° to +30°. Asked for
//! between 7.5 and 12.5 from the z axis, from z = −0.5 to 0.5 along it (a prism's sweep is asked
//! for between its ends) and from −10° to 70°, which holds the notch; cut from the block, the
//! rectangle 8 ≤ x ≤ 12, |z| ≤ 1 revolved from 0° to 60°.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, revolve_profile_brep, sweep_solid_within, BooleanOperation, BrepSolid,
    OutlinePiece, ScrewStep, SolidMotion, SweepClip, SweptTool,
};
#[allow(unused_imports)]
use std::f64::consts::{PI, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let section = [p(8.0, 0.0, -1.0), p(12.0, 0.0, -1.0), p(12.0, 0.0, 1.0), p(8.0, 0.0, 1.0)];
    let block = revolve_profile_brep(&polygon(&section)?, p(0.0, 0.0, 0.0), z, 60f64.to_radians())?;
    let tool = SweptTool::Prism {
        origin: p(0.0, 0.0, 0.0),
        normal: z,
        x_axis: p(1.0, 0.0, 0.0),
        outline: vec![OutlinePiece::Arc { centre: [10.0, 0.0], radius: 1.0, start: 0.0, end: TAU }],
        from: -0.5,
        to: 0.5,
    };
    let turn = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: z, turn: 1.0, advance: 0.0 }],
        from: (-140f64).to_radians(),
        to: 30f64.to_radians(),
    };
    let around_notch = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: z,
        reference: p(1.0, 0.0, 0.0),
        radius: [7.5, 12.5],
        axial: [-0.5, 0.5],
        angle: Some([(-10f64).to_radians(), 70f64.to_radians()]),
    };
    let mut operands = vec![block];
    operands.extend(sweep_solid_within(&tool, &turn, &around_notch)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

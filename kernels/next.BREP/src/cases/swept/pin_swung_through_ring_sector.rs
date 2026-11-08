//! A ring sector slotted by a pin swung through it.
//!
//! The kernel's sweep of a solid (`sweep_solid_within`): the pin is a solid of revolution about the
//! x axis, its meridian the rectangle out to radius 0.5 from x = 6 to x = 11, turned about the z
//! axis from −140° to +140°. Asked for between 7.5 and 12.5 from the z axis, from z = −1.5 to 1.5
//! and from −10° to 70°, which holds the block (the rectangle 8 ≤ x ≤ 12, |z| ≤ 1 revolved from 0°
//! to 60°) and which neither end position reaches; cut from the block.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, revolve_profile_brep, sweep_solid_within, BooleanOperation, BrepSolid,
    OutlinePiece, ScrewStep, SolidMotion, SweepClip, SweptTool,
};
#[allow(unused_imports)]
use std::f64::consts::{FRAC_PI_2, PI, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let section = [p(8.0, 0.0, -1.0), p(12.0, 0.0, -1.0), p(12.0, 0.0, 1.0), p(8.0, 0.0, 1.0)];
    let block = revolve_profile_brep(&polygon(&section)?, p(0.0, 0.0, 0.0), z, 60f64.to_radians())?;
    let tool = SweptTool::Revolved {
        axis_origin: p(6.0, 0.0, 0.0),
        axis_direction: p(1.0, 0.0, 0.0),
        radial: p(0.0, 0.0, 1.0),
        meridian: vec![
            OutlinePiece::Line { from: [0.0, 0.0], to: [0.5, 0.0] },
            OutlinePiece::Line { from: [0.5, 0.0], to: [0.5, 5.0] },
            OutlinePiece::Line { from: [0.5, 5.0], to: [0.0, 5.0] },
            OutlinePiece::Line { from: [0.0, 5.0], to: [0.0, 0.0] },
        ],
    };
    let motion = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: z, turn: 1.0, advance: 0.0 }],
        from: (-140.0f64).to_radians(),
        to: 140.0f64.to_radians(),
    };
    let around_block = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: z,
        reference: p(1.0, 0.0, 0.0),
        radius: [7.5, 12.5],
        axial: [-1.5, 1.5],
        angle: Some([(-10f64).to_radians(), 70f64.to_radians()]),
    };
    let mut operands = vec![block];
    operands.extend(sweep_solid_within(&tool, &motion, &around_block)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

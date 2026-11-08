//! A ring sector grooved by a ball-end mill screwed across it.
//!
//! The kernel's sweep of a solid (`sweep_solid_within`): the mill is a solid of revolution about
//! the vertical line through (10, 0), its meridian the quarter circle of radius 1 below its centre
//! (10, 0, 0), the side up to z = 3 and the flat top; swept by the screw about the z axis that
//! rises 3 a turn, from −60° to +60°. Asked for between 7.5 and 12.5 from the z axis, from z = −2.5
//! to 2.5 and from −35° to 35°, which holds the block and which neither end position reaches; cut
//! from the block, the rectangle 8 ≤ x ≤ 12, |z| ≤ 2 revolved from −30° to 30°.
use crate::solids::sweep::{moved, turn_z};
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, revolve_profile_brep, sweep_solid_within, BooleanOperation, BrepSolid,
    OutlinePiece, ScrewStep, SolidMotion, SweepClip, SweptTool,
};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let section = [p(8.0, 0.0, -2.0), p(12.0, 0.0, -2.0), p(12.0, 0.0, 2.0), p(8.0, 0.0, 2.0)];
    let sector = revolve_profile_brep(&polygon(&section)?, p(0.0, 0.0, 0.0), z, 60f64.to_radians())?;
    let block = moved(&sector, turn_z(-30f64.to_radians(), p(0.0, 0.0, 0.0))?)?;
    let mill = SweptTool::Revolved {
        axis_origin: p(10.0, 0.0, 0.0),
        axis_direction: z,
        radial: p(1.0, 0.0, 0.0),
        meridian: vec![
            OutlinePiece::Arc { centre: [0.0, 0.0], radius: 1.0, start: -FRAC_PI_2, end: 0.0 },
            OutlinePiece::Line { from: [1.0, 0.0], to: [1.0, 3.0] },
            OutlinePiece::Line { from: [1.0, 3.0], to: [0.0, 3.0] },
            OutlinePiece::Line { from: [0.0, 3.0], to: [0.0, -1.0] },
        ],
    };
    let screw = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: z, turn: 1.0, advance: 3.0 / TAU }],
        from: (-60f64).to_radians(),
        to: 60f64.to_radians(),
    };
    let around_block = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: z,
        reference: p(1.0, 0.0, 0.0),
        radius: [7.5, 12.5],
        axial: [-2.5, 2.5],
        angle: Some([(-35f64).to_radians(), 35f64.to_radians()]),
    };
    let mut operands = vec![block];
    operands.extend(sweep_solid_within(&mill, &screw, &around_block)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

//! A rod threaded by the flat end of an end mill carried along a screw about its axis.
//!
//! The kernel's sweep of a solid (`sweep_solid_within`): the mill is a solid of revolution about
//! the x axis, its meridian the rectangle out to radius 1.2 from x = 4 to x = 12, swept by the
//! screw about the z axis that rises 6 a turn, from −90° to +1350°. Asked for within 5.2 of the z
//! axis from z = −0.2 to 20.2, which holds the rod and which neither end position reaches (lowered
//! 1.5, the mill reaches up to z = −0.3); cut from the rod.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_cylinder_brep, sweep_solid_within, BooleanOperation, BrepSolid,
    OutlinePiece, ScrewStep, SolidMotion, SweepClip, SweptTool,
};
#[allow(unused_imports)]
use std::f64::consts::{PI, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), z, 5.0, 20.0)?;
    let tool = SweptTool::Revolved {
        axis_origin: p(4.0, 0.0, 0.0),
        axis_direction: p(1.0, 0.0, 0.0),
        radial: p(0.0, 0.0, 1.0),
        meridian: vec![
            OutlinePiece::Line { from: [0.0, 0.0], to: [1.2, 0.0] },
            OutlinePiece::Line { from: [1.2, 0.0], to: [1.2, 8.0] },
            OutlinePiece::Line { from: [1.2, 8.0], to: [0.0, 8.0] },
            OutlinePiece::Line { from: [0.0, 8.0], to: [0.0, 0.0] },
        ],
    };
    let screw = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: z, turn: 1.0, advance: 6.0 / TAU }],
        from: (-90f64).to_radians(),
        to: 1350f64.to_radians(),
    };
    let around_rod = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: z,
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 5.2],
        axial: [-0.2, 20.2],
        angle: None,
    };
    let mut operands = vec![rod];
    operands.extend(sweep_solid_within(&tool, &screw, &around_rod)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

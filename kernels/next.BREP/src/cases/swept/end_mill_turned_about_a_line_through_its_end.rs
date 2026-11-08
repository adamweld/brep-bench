//! A block pocketed by an end mill turned about a line through its own flat end.
//!
//! The kernel's sweep of a solid (`sweep_solid_within`, its planar motion of a prism, which
//! includes the end positions exactly): the mill is the prism of the circle of radius 3 about (1.5,
//! 0) from z = −2 to z = 4, turned 120° about the z axis. Asked for within 14.5 of the z axis from
//! z = −2 to 4 (a prism's sweep is asked for between its ends), which holds the pocket; cut from
//! the block.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_box_brep, sweep_solid_within, BooleanOperation, BrepSolid,
    OutlinePiece, ScrewStep, SolidMotion, SweepClip, SweptTool,
};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let block = make_box_brep(p(-10.0, -10.0, -5.0), 20.0, 20.0, 5.0)?;
    let mill = SweptTool::Prism {
        origin: p(0.0, 0.0, 0.0),
        normal: z,
        x_axis: p(1.0, 0.0, 0.0),
        outline: vec![OutlinePiece::Arc { centre: [1.5, 0.0], radius: 3.0, start: 0.0, end: TAU }],
        from: -2.0,
        to: 4.0,
    };
    let turn = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: z, turn: 1.0, advance: 0.0 }],
        from: 0.0,
        to: 120f64.to_radians(),
    };
    let around_block = SweepClip {
        axis_origin: p(0.0, -30.0, 0.0),
        axis_direction: z,
        reference: p(1.0, 0.0, 0.0),
        radius: [19.0, 42.0],
        axial: [-2.0, 4.0],
        angle: None,
    };
    let mut operands = vec![block];
    operands.extend(sweep_solid_within(&mill, &turn, &around_block)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

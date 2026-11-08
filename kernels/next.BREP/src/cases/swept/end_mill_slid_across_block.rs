//! A block slotted across its top by an end mill slid past it.
//!
//! The kernel's sweep of a solid (`sweep_solid_within`, its planar motion of a prism, which
//! includes the end positions exactly): the mill is the prism of the circle of radius 2 about (−5,
//! 0) from z = 7 to z = 15, slid 30 along the x axis. Asked for between 24 and 37 from the vertical
//! line through (10, −30), an axis off the sweep,, from z = 7 to 15 (a prism's sweep is asked for
//! between its ends), which holds the slot; cut from the block.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_box_brep, sweep_solid_within, BooleanOperation, BrepSolid,
    OutlinePiece, ScrewStep, SolidMotion, SweepClip, SweptTool,
};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let block = make_box_brep(p(0.0, -5.0, 0.0), 20.0, 10.0, 10.0)?;
    let mill = SweptTool::Prism {
        origin: p(0.0, 0.0, 0.0),
        normal: z,
        x_axis: p(1.0, 0.0, 0.0),
        outline: vec![OutlinePiece::Arc { centre: [-5.0, 0.0], radius: 2.0, start: 0.0, end: TAU }],
        from: 7.0,
        to: 15.0,
    };
    let slide = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(1.0, 0.0, 0.0), turn: 0.0, advance: 30.0 }],
        from: 0.0,
        to: 1.0,
    };
    let around_block = SweepClip {
        axis_origin: p(10.0, -30.0, 0.0),
        axis_direction: z,
        reference: p(1.0, 0.0, 0.0),
        radius: [24.0, 37.0],
        axial: [7.0, 15.0],
        angle: None,
    };
    let mut operands = vec![block];
    operands.extend(sweep_solid_within(&mill, &slide, &around_block)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

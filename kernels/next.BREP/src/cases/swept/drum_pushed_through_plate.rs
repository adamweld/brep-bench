//! A plate slotted by a drum, lying on its side, pushed straight up through it.
//!
//! The kernel's sweep of a solid (`sweep_solid_within`, its planar motion of a prism, which
//! includes the end positions exactly): the drum is the prism along the x axis of the circle of
//! radius 2 about (y, z) = (0, −5), from x = 6 to x = 14, slid 16 up the z axis, square to its
//! length. Asked for within 5.5 of the line y = 0, z = 2 from x = 6 to 14 (a prism's sweep is asked
//! for between its ends), which holds the slot; cut from the plate.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_box_brep, sweep_solid_within, BooleanOperation, BrepSolid,
    OutlinePiece, ScrewStep, SolidMotion, SweepClip, SweptTool,
};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let x = p(1.0, 0.0, 0.0);
    let plate = make_box_brep(p(0.0, -5.0, 0.0), 20.0, 10.0, 4.0)?;
    // The prism's plane coordinates run along y and along x × y = z.
    let drum = SweptTool::Prism {
        origin: p(0.0, 0.0, 0.0),
        normal: x,
        x_axis: p(0.0, 1.0, 0.0),
        outline: vec![OutlinePiece::Arc { centre: [0.0, -5.0], radius: 2.0, start: 0.0, end: TAU }],
        from: 6.0,
        to: 14.0,
    };
    let push = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 0.0, advance: 16.0 }],
        from: 0.0,
        to: 1.0,
    };
    let around_plate = SweepClip {
        axis_origin: p(0.0, -30.0, 2.0),
        axis_direction: x,
        reference: p(0.0, 1.0, 0.0),
        radius: [24.5, 35.5],
        axial: [6.0, 14.0],
        angle: None,
    };
    let mut operands = vec![plate];
    operands.extend(sweep_solid_within(&drum, &push, &around_plate)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

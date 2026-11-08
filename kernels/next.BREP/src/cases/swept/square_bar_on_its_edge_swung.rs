//! Everything a square bar, stood on an edge facing the z axis, passes through as it swings a
//! quarter turn about that axis.
//!
//! The kernel's sweep of a solid (`sweep_solid_within`, its planar motion of a prism, which
//! includes the end positions exactly): the bar is the prism of the square with corners (4, 0), (6,
//! −2), (8, 0) and (6, 2) from z = 0 to z = 3, turned 90° about the z axis. Asked for within 40 of
//! the vertical line through (−10, −10) from z = 0 to 3 (a prism's sweep is asked for between its
//! ends), which holds the whole sweep; its axis is kept off the turn's, about which the bar's
//! corners keep their distance.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, sweep_solid_within, BooleanOperation, BrepSolid, OutlinePiece,
    ScrewStep, SolidMotion, SweepClip, SweptTool,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let corners = [[4.0, 0.0], [6.0, -2.0], [8.0, 0.0], [6.0, 2.0]];
    let bar = SweptTool::Prism {
        origin: p(0.0, 0.0, 0.0),
        normal: z,
        x_axis: p(1.0, 0.0, 0.0),
        outline: (0..4).map(|k| OutlinePiece::Line { from: corners[k], to: corners[(k + 1) % 4] }).collect(),
        from: 0.0,
        to: 3.0,
    };
    let turn = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: z, turn: 1.0, advance: 0.0 }],
        from: 0.0,
        to: 90f64.to_radians(),
    };
    let around = SweepClip {
        axis_origin: p(-10.0, -10.0, 0.0),
        axis_direction: z,
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 40.0],
        axial: [0.0, 3.0],
        angle: None,
    };
    let bodies = sweep_solid_within(&bar, &turn, &around)?;
    Ok(boolean_operation_nary(&bodies, BooleanOperation::Union)?)
}

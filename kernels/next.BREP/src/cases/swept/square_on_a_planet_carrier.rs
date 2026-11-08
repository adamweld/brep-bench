//! Everything a square bar passes through as it is carried round on a planet, turning about its own
//! centre against the carrier.
//!
//! The kernel's sweep of a solid (`sweep_solid_within`, its planar motion of a prism): the bar is
//! the prism of the square of side 2 about (5, 0) from z = 0 to z = 2. At t its centre is carried
//! by t about the z axis and the bar turns by −2t about its centre, so it is first turned by −3t
//! about the vertical line through (5, 0) and then by t about the z axis, for t from 0° to 360°.
//! Asked for within 9 of the z axis from z = 0 to 2 (a prism's sweep is asked for between its
//! ends), which holds the whole sweep.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, sweep_solid_within, BooleanOperation, BrepSolid, OutlinePiece,
    ScrewStep, SolidMotion, SweepClip, SweptTool,
};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let corners = [[4.0, -1.0], [6.0, -1.0], [6.0, 1.0], [4.0, 1.0]];
    let bar = SweptTool::Prism {
        origin: p(0.0, 0.0, 0.0),
        normal: z,
        x_axis: p(1.0, 0.0, 0.0),
        outline: (0..4).map(|k| OutlinePiece::Line { from: corners[k], to: corners[(k + 1) % 4] }).collect(),
        from: 0.0,
        to: 2.0,
    };
    let planet = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(5.0, 0.0, 0.0), axis_direction: z, turn: -3.0, advance: 0.0 },
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: z, turn: 1.0, advance: 0.0 },
        ],
        from: 0.0,
        to: TAU,
    };
    let around = SweepClip {
        axis_origin: p(-20.0, 0.0, 0.0),
        axis_direction: z,
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 30.0],
        axial: [0.0, 2.0],
        angle: None,
    };
    let bodies = sweep_solid_within(&bar, &planet, &around)?;
    Ok(boolean_operation_nary(&bodies, BooleanOperation::Union)?)
}

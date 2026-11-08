//! A rod threaded by a bar carried along a screw so fine that the groove's turns meet.
//!
//! Through the kernel's sweep of a solid (`sweep_solid_within`), asked for the bar as a prism (its
//! rectangle 4.5 ≤ x ≤ 12, |y| ≤ 8 from z = −0.45 to 0.45) along the screw about the z axis that
//! rises 1 a turn, from −180° to +7380°, within 5.2 of the z axis from z = −0.02 to 20.02 (which
//! holds the rod and which neither end position reaches); cut from the rod. The kernel's sweep
//! carries a solid of revolution along a screw and a prism only along a planar motion, so it may
//! refuse.
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
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let mut operands = vec![rod];
    let tool0 = SweptTool::Prism {
        origin: p(0.0, 0.0, 0.0),
        normal: p(0.0, 0.0, 1.0),
        x_axis: p(1.0, 0.0, 0.0),
        outline: vec![
            OutlinePiece::Line { from: [4.5, -8.0], to: [12.0, -8.0] },
            OutlinePiece::Line { from: [12.0, -8.0], to: [12.0, 8.0] },
            OutlinePiece::Line { from: [12.0, 8.0], to: [4.5, 8.0] },
            OutlinePiece::Line { from: [4.5, 8.0], to: [4.5, -8.0] },
        ],
        from: -0.45,
        to: 0.45,
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 1.0 / TAU },
        ],
        from: (-180f64).to_radians(),
        to: 7380f64.to_radians(),
    };
    let clip0 = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 5.2],
        axial: [-0.02, 20.02],
        angle: None,
    };
    operands.extend(sweep_solid_within(&tool0, &motion0, &clip0)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

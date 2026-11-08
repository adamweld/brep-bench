//! A rod with a sealed helical tunnel: everything a small box passes through along a screw inside
//! it.
//!
//! Through the kernel's sweep of a solid (`sweep_solid_within`), asked for the box as a prism (its
//! square 2.5 ≤ x ≤ 3.5, |y| ≤ 0.5 from z = 2 to 3) along the screw about the z axis that rises 4 a
//! turn, from 0° to 720°, within 5.2 of the z axis from z = −0.4 to 20.4; cut from the rod. The
//! kernel's sweep takes a prism only along a planar motion and a screw only where neither end
//! position reaches the region asked for, so it may refuse.
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
            OutlinePiece::Line { from: [2.5, -0.5], to: [3.5, -0.5] },
            OutlinePiece::Line { from: [3.5, -0.5], to: [3.5, 0.5] },
            OutlinePiece::Line { from: [3.5, 0.5], to: [2.5, 0.5] },
            OutlinePiece::Line { from: [2.5, 0.5], to: [2.5, -0.5] },
        ],
        from: 2.0,
        to: 3.0,
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 4.0 / TAU },
        ],
        from: 0.0,
        to: (4.0 * PI),
    };
    let clip0 = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 5.2],
        axial: [-0.4, 20.4],
        angle: None,
    };
    operands.extend(sweep_solid_within(&tool0, &motion0, &clip0)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

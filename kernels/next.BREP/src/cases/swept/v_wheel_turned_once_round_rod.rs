//! A rod grooved all round by a leaning wheel with a sharp V rim, turned once round it.
//!
//! Through the kernel's sweep of a solid (`sweep_solid_within`): the wheel is a solid of revolution
//! about the line through (14, 0, 10) along (0, sin 20°, cos 20°), its meridian the triangle with
//! its tip 10 from the axis (at (4, 0, 10)) and its other corners 7.5 from it, ±2.5 tan 30° along
//! it; turned a whole turn about the z axis, asked for within 5.2 of the z axis from z = −0.5 to
//! 20.5, which holds the rod; cut from the rod. The kernel's sweep takes a turn only where neither
//! end position reaches the region asked for, so it may refuse.
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
    let flank = 2.5 * (PI / 6.0).tan();
    let mut operands = vec![rod];
    let tool0 = SweptTool::Revolved {
        axis_origin: p(14.0, 0.0, 10.0),
        axis_direction: p(0.0, 20f64.to_radians().sin(), 20f64.to_radians().cos()),
        radial: p(-1.0, 0.0, 0.0),
        meridian: vec![
            OutlinePiece::Line { from: [10.0, 0.0], to: [7.5, flank] },
            OutlinePiece::Line { from: [7.5, flank], to: [7.5, -flank] },
            OutlinePiece::Line { from: [7.5, -flank], to: [10.0, 0.0] },
        ],
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 0.0 },
        ],
        from: 0.0,
        to: TAU,
    };
    let clip0 = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 5.2],
        axial: [-0.5, 20.5],
        angle: None,
    };
    operands.extend(sweep_solid_within(&tool0, &motion0, &clip0)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

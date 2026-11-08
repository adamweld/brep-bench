//! A ring sector whose floor is cut by the flat end of a leaning bar turned past it.
//!
//! Through the kernel's sweep of a solid (`sweep_solid_within`), asked for the bar as a prism: its
//! end, the rectangle {0 ≤ a cos 5° + b sin 5° ≤ 3, |b cos 5° − a sin 5°| ≤ 2} of the plane through
//! (10, 0, 0) along u = (cos 20°, 0, −sin 20°) and v = (0, 1, 0), carried 3 along n = (sin 20°, 0,
//! cos 20°); turned about the z axis from −140° to +140°, within 7.5 to 12.5 of the z axis, from z
//! = −1.5 to 1.5 and from −10° to 70°; cut from the block (the rectangle 8 ≤ x ≤ 12, |z| ≤ 1
//! revolved from 0° to 60°). The prism leans to the turn's axis, so this is no planar motion of it,
//! and the kernel's sweep may refuse.
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
    let section = [p(8.0, 0.0, -1.0), p(12.0, 0.0, -1.0), p(12.0, 0.0, 1.0), p(8.0, 0.0, 1.0)];
    let block = revolve_profile_brep(&crate::solids::polygon(&section)?, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 60f64.to_radians())?;
    let (c, s) = (5f64.to_radians().cos(), 5f64.to_radians().sin());
    // a corner (a', b') of the turned rectangle, in the plane's own (a, b)
    let corner = |a: f64, b: f64| [a * c - b * s, a * s + b * c];
    let corners = [corner(0.0, -2.0), corner(3.0, -2.0), corner(3.0, 2.0), corner(0.0, 2.0)];
    let lean = 20f64.to_radians();
    let bar = SweptTool::Prism {
        origin: p(10.0, 0.0, 0.0),
        normal: p(lean.sin(), 0.0, lean.cos()),
        x_axis: p(lean.cos(), 0.0, -lean.sin()),
        outline: (0..4).map(|k| OutlinePiece::Line { from: corners[k], to: corners[(k + 1) % 4] }).collect(),
        from: 0.0,
        to: 3.0,
    };
    let mut operands = vec![block];
    let tool0 = bar.clone();
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 0.0 },
        ],
        from: (-140f64).to_radians(),
        to: 140f64.to_radians(),
    };
    let clip0 = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [7.5, 12.5],
        axial: [-1.5, 1.5],
        angle: Some([(-10f64).to_radians(), 70f64.to_radians()]),
    };
    operands.extend(sweep_solid_within(&tool0, &motion0, &clip0)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

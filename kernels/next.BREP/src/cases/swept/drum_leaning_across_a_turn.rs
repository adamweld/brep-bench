//! A ring with a sealed channel cut inside it by a leaning drum turned 100° about its axis.
//!
//! Through the kernel's sweep of a solid (`sweep_solid_within`): the drum is a solid of revolution
//! about the line through (5, 1.2, 0) along (sin 25°, 0, cos 25°), its meridian the rectangle out
//! to radius 1 from 1.5 below its middle to 1.5 above; turned about the z axis from 0° to 100°,
//! asked for between 3 and 8 from the z axis from z = −2.5 to 2.5, which holds the ring (the
//! cylinder of radius 8 less that of radius 3, from z = −2.5 to 2.5); cut from the ring. The
//! kernel's sweep takes a turn only where neither end position reaches the region asked for, so it
//! may refuse.
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
    let outer = make_cylinder_brep(p(0.0, 0.0, -2.5), p(0.0, 0.0, 1.0), 8.0, 5.0)?;
    let inner = make_cylinder_brep(p(0.0, 0.0, -3.0), p(0.0, 0.0, 1.0), 3.0, 6.0)?;
    let ring = boolean_operation_nary(&[outer, inner], BooleanOperation::Subtract)?;
    let mut operands = vec![ring];
    let tool0 = SweptTool::Revolved {
        axis_origin: p(5.0, 1.2, 0.0),
        axis_direction: p(25f64.to_radians().sin(), 0.0, 25f64.to_radians().cos()),
        radial: p(0.0, 1.0, 0.0),
        meridian: vec![
            OutlinePiece::Line { from: [0.0, -1.5], to: [1.0, -1.5] },
            OutlinePiece::Line { from: [1.0, -1.5], to: [1.0, 1.5] },
            OutlinePiece::Line { from: [1.0, 1.5], to: [0.0, 1.5] },
            OutlinePiece::Line { from: [0.0, 1.5], to: [0.0, -1.5] },
        ],
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 0.0 },
        ],
        from: 0.0,
        to: 100f64.to_radians(),
    };
    let clip0 = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [3.0, 8.0],
        axial: [-2.5, 2.5],
        angle: None,
    };
    operands.extend(sweep_solid_within(&tool0, &motion0, &clip0)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

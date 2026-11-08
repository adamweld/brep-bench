//! A plate pocketed from below by a drum lying on its side, pushed up aslant and stopping inside.
//!
//! Through the kernel's sweep of a solid (`sweep_solid_within`): the drum is a solid of revolution
//! about the line y = 0, z = −5 along x, its meridian the rectangle out to radius 2 from x = 6 to x
//! = 14; slid 8 along (0.3, 0, √(1 − 0.3²)), asked for within 5.5 of the line y = 0, z = 2 from x =
//! −0.5 to 20.5, which holds the plate; cut from the plate. The kernel's sweep takes a solid of
//! revolution only along a screw or turn whose end positions stay out of the region asked for, so
//! it may refuse.
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
    let plate = make_box_brep(p(0.0, -5.0, 0.0), 20.0, 10.0, 4.0)?;
    let mut operands = vec![plate];
    let tool0 = SweptTool::Revolved {
        axis_origin: p(6.0, 0.0, -5.0),
        axis_direction: p(1.0, 0.0, 0.0),
        radial: p(0.0, 0.0, 1.0),
        meridian: vec![
            OutlinePiece::Line { from: [0.0, 0.0], to: [2.0, 0.0] },
            OutlinePiece::Line { from: [2.0, 0.0], to: [2.0, 8.0] },
            OutlinePiece::Line { from: [2.0, 8.0], to: [0.0, 8.0] },
            OutlinePiece::Line { from: [0.0, 8.0], to: [0.0, 0.0] },
        ],
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.3, 0.0, (1.0f64 - 0.09).sqrt()), turn: 0.0, advance: 8.0 },
        ],
        from: 0.0,
        to: 1.0,
    };
    let clip0 = SweepClip {
        axis_origin: p(0.0, 0.0, 2.0),
        axis_direction: p(1.0, 0.0, 0.0),
        reference: p(0.0, 1.0, 0.0),
        radius: [0.0, 5.5],
        axial: [-0.5, 20.5],
        angle: None,
    };
    operands.extend(sweep_solid_within(&tool0, &motion0, &clip0)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

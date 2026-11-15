//! A rod threaded by a drafted slot-section bar carried along a screw.
//!
//! Through the kernel's sweep of a solid (`sweep_solid_within`), asked for the bar (its slot in
//! the plane x = 12, two lines and two half circles, carried 8 along −x and drafted 3° inward)
//! along the screw about the z axis that rises 6 a turn, from −90° to +1350°, within 5.2 of the
//! z axis from z = −0.4 to 20.4 (which holds the rod and which neither end position reaches); cut
//! from the rod.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_arc, make_cylinder_brep, make_line, sweep_solid_within, BooleanOperation, BrepSolid, ScrewStep,
    SolidMotion, SweepClip, SweptTool,
};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let (y, z) = (p(0.0, 1.0, 0.0), p(0.0, 0.0, 1.0));
    let bar = SweptTool::Bar {
        origin: p(12.0, 0.0, 0.0),
        normal: p(-1.0, 0.0, 0.0),
        section: vec![
            make_line(p(12.0, -1.5, -1.0), p(12.0, 1.5, -1.0))?,
            make_arc(p(12.0, 1.5, 0.0), y, z, 1.0, -FRAC_PI_2, FRAC_PI_2)?,
            make_line(p(12.0, 1.5, 1.0), p(12.0, -1.5, 1.0))?,
            make_arc(p(12.0, -1.5, 0.0), y, z, 1.0, FRAC_PI_2, 3.0 * FRAC_PI_2)?,
        ],
        from: 0.0,
        to: 8.0,
        draft: 3.0f64.to_radians(),
    };
    let screw = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 6.0 / TAU }],
        from: (-90.0f64).to_radians(),
        to: 1350.0f64.to_radians(),
    };
    let clip = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 5.2],
        axial: [-0.4, 20.4],
        angle: None,
    };
    let mut operands = vec![rod];
    operands.extend(sweep_solid_within(&bar, &screw, &clip)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

//! A rod threaded by a bar whose section is a closed periodic cubic B-spline, carried along a
//! screw.
//!
//! Through the kernel's sweep of a solid (`sweep_solid_within`), asked for the bar (the kernel's
//! periodic B-spline, `make_periodic_bspline`, on the seven control points in the plane y = 0,
//! from y = −8 to 8) along the screw about the z axis that rises 6 a turn, from −90° to +1350°,
//! within 5.2 of the z axis from z = −0.05 to 20.05 (which holds the rod and which neither end
//! position reaches: at the start the bar's top is at z = −0.08 or lower); cut from the rod.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_cylinder_brep, make_periodic_bspline, sweep_solid_within, BooleanOperation, BrepSolid, ScrewStep,
    SolidMotion, SweepClip, SweptTool,
};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let poles = [(3.5, 0.0), (4.5, -1.5), (8.0, -1.5), (12.0, -1.0), (12.0, 1.0), (8.0, 1.5), (4.5, 1.5)].map(|(x, z)| p(x, 0.0, z));
    let bar = SweptTool::Bar {
        origin: p(0.0, 0.0, 0.0),
        normal: p(0.0, 1.0, 0.0),
        section: vec![make_periodic_bspline(3, &poles)?],
        from: -8.0,
        to: 8.0,
        draft: 0.0,
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
        axial: [-0.05, 20.05],
        angle: None,
    };
    let mut operands = vec![rod];
    operands.extend(sweep_solid_within(&bar, &screw, &clip)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

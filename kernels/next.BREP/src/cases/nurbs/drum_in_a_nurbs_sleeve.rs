//! The solid cylinder of radius 5 about the z axis from z = 0 to z = 10 united with a sleeve, the
//! ring between 5 and 7 from the axis with 2 ≤ z ≤ 8, whose bore is a NURBS cylinder.
//!
//! The sleeve is the circular cylinder of radius 7 from z = 2 to z = 8 less the solid whose side is
//! the case's NURBS bore, the rational quadratic circle of radius 5 starting at (0, 5) times the
//! line from z = 2 to z = 8 (`solids::nurbs::nurbs_cylinder`), so the sleeve's bore is that face.
use crate::solids::fillet::{subtract, union};
use crate::solids::nurbs::nurbs_cylinder;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let up = p(0.0, 0.0, 1.0);
    let drum = make_cylinder_brep(p(0.0, 0.0, 0.0), up, 5.0, 10.0)?;
    // the circle's control points from (0, 1) counter-clockwise, as the case lists them
    let from_y = [(0.0, 1.0), (-1.0, 1.0), (-1.0, 0.0), (-1.0, -1.0), (0.0, -1.0), (1.0, -1.0),
        (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let bore = nurbs_cylinder(5.0, 2.0, 8.0, &from_y)?;
    let outside = make_cylinder_brep(p(0.0, 0.0, 2.0), up, 7.0, 6.0)?;
    let sleeve = subtract(&outside, &bore)?;
    Ok(union(&drum, &sleeve)?)
}

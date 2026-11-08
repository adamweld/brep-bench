//! The plate from (0, −5, 0) to (20, 5, 4), less everything a square bar stood on a corner
//! (6 ≤ x ≤ 14, |y| + |z + 6| ≤ 1) passes through as it slides without turning by 8 along
//! (0.3, 0, √0.91), stopping inside the plate.
//!
//! next.BREP's sweep of a solid takes a prism only in motions square to its own direction, and
//! this slide is not. A solid slid along a line is, by elementary geometry
//! (`solids::swept::polyhedron_slid`), the solid where it starts and where it stops and each of
//! its faces slid along the line: the bar is the kernel's extrude of its square section along x,
//! each face slid is that face extruded along the slide, and all of it is cut from the plate.
use crate::solids::p;
use crate::solids::swept::polyhedron_slid;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, extrude_profile_brep, make_box_brep, BooleanOperation, BrepSolid,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(0.0, -5.0, 0.0), 20.0, 10.0, 4.0)?;
    let section = |x: f64| [p(x, 0.0, -7.0), p(x, 1.0, -6.0), p(x, 0.0, -5.0), p(x, -1.0, -6.0)];
    let (near, far) = (section(6.0), section(14.0));
    let bar = extrude_profile_brep(&crate::solids::polygon(&near)?, p(1.0, 0.0, 0.0), 8.0)?;
    let mut faces = vec![near.to_vec(), far.to_vec()];
    for k in 0..4 {
        faces.push(vec![near[k], near[(k + 1) % 4], far[(k + 1) % 4], far[k]]);
    }
    let push = p(0.3, 0.0, (1.0f64 - 0.09).sqrt()).scale(8.0);
    let mut operands = vec![plate];
    operands.extend(polyhedron_slid(&bar, &faces, push)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

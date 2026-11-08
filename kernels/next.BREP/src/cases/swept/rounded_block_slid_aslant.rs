//! Everything the box from (0, 0, 0) to (10, 6, 4) with every edge and corner rounded to radius 1
//! (everything within 1 of the box from (1, 1, 1) to (9, 5, 3)) passes through as it slides
//! without turning along (8, 5, 3).
//!
//! next.BREP has no sweep of a solid along a line. The rounded block is the inner box grown by 1,
//! and growing commutes with sliding (a point within 1 of the inner box at some place of the slide
//! is within 1 of the inner box's sweep, and back), so the sweep is the inner box's sweep grown
//! by 1. The inner box's sweep is, by elementary geometry (`solids::swept::polyhedron_slid`), the
//! box where it starts and where it stops and each of its faces slid along the line, united by the
//! kernel; the kernel's offset grows it by 1, rounding every edge and corner.
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::solids::swept::{box_faces, polyhedron_slid};
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, make_box_brep, BooleanOperation, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (corner, size) = (p(1.0, 1.0, 1.0), p(8.0, 4.0, 2.0));
    let inner = make_box_brep(corner, size.x, size.y, size.z)?;
    let faces = box_faces(corner, p(size.x, 0.0, 0.0), p(0.0, size.y, 0.0), p(0.0, 0.0, size.z));
    let slid = boolean_operation_nary(&polyhedron_slid(&inner, &faces, p(8.0, 5.0, 3.0))?,
        BooleanOperation::Union)?;
    Ok(parallel(&slid, 1.0)?)
}

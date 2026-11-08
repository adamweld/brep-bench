//! The plate from (−15, −15, 0) to (15, 15, 4), less everything a tilted box passes through as it
//! slides without turning along (4, 2, 18): the box (−2, −1, −8) + a e₁ + b e₂ + c e₃, |a| ≤ 2,
//! |b| ≤ 1.5, |c| ≤ 1, e₁ = (2, 1, 2)/3, e₂ = (−2, 2, 1)/3, e₃ = (1, 2, −2)/3.
//!
//! next.BREP's sweep of a solid takes a prism only in motions square to its own direction, and no
//! edge of this box is square to the slide. A solid slid along a line is, by elementary geometry
//! (`solids::swept::polyhedron_slid`), the solid where it starts and where it stops and each of
//! its faces slid along the line: the box is the kernel's box turned onto the edges e₁, e₂, −e₃
//! (a rotation; the box is symmetric in c) and moved, each face slid is that face extruded along
//! the slide, and all of it is cut from the plate.
use crate::solids::p;
use crate::solids::swept::{box_faces, polyhedron_slid};
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_box_brep, transform_brep, AffineTransform, BooleanOperation,
    BrepSolid,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-15.0, -15.0, 0.0), 30.0, 30.0, 4.0)?;
    let e1 = p(2.0, 1.0, 2.0).scale(1.0 / 3.0);
    let e2 = p(-2.0, 2.0, 1.0).scale(1.0 / 3.0);
    let e3 = p(-1.0, -2.0, 2.0).scale(1.0 / 3.0); // −e₃, so that e₁, e₂, −e₃ turn right-handed
    let centre = p(-2.0, -1.0, -8.0);
    let corner = centre.sub(e1.scale(2.0)).sub(e2.scale(1.5)).sub(e3.scale(1.0));
    let placed = AffineTransform::new([
        e1.x, e2.x, e3.x, corner.x, //
        e1.y, e2.y, e3.y, corner.y, //
        e1.z, e2.z, e3.z, corner.z, //
        0.0, 0.0, 0.0, 1.0,
    ])?;
    let tilted = transform_brep(&make_box_brep(p(0.0, 0.0, 0.0), 4.0, 3.0, 2.0)?, placed, false)?;
    let faces = box_faces(corner, e1.scale(4.0), e2.scale(3.0), e3.scale(2.0));
    let mut operands = vec![plate];
    operands.extend(polyhedron_slid(&tilted, &faces, p(4.0, 2.0, 18.0))?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

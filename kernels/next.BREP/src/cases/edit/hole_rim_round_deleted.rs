//! The box from the origin to (20, 10, 10) drilled through by a hole of radius 2 about the line
//! x = 10, y = 5, the hole's top rim rounded to radius 1; then the round, a quarter torus, deleted,
//! the top and the hole's wall run on to meet in the rim again: the drilled block.
//!
//! The solid is made as the case says (the hole a cylinder cut from the box); then the kernel's
//! `delete_faces_and_heal` deletes the face.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{
    delete_faces_and_heal, fillet_edges, make_box_brep, make_cylinder_brep, BrepSolid,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let block = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 10.0, 10.0)?;
    let drilled = cut(&block, &make_cylinder_brep(p(10.0, 5.0, -1.0), z, 2.0, 12.0)?)?;
    let rounded = fillet_edges(&drilled, &[p(10.0, 3.0, 10.0)], None, 1.0, false, None)?;
    // the round at 45° round its tube, whose center line is the circle of radius 3 at z = 9
    let s = 2f64.sqrt() / 2.0;
    let round = face_at(&rounded, p(10.0, 5.0 - (3.0 - s), 9.0 + s))?;
    Ok(delete_faces_and_heal(&rounded, &[round])?)
}

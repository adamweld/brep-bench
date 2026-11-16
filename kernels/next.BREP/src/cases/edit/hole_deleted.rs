//! The box from the origin to (20, 10, 10), drilled through along z by a hole of radius 3 through
//! (10, 5); then the hole's wall deleted, the top and the bottom closing over its two ends: the
//! box.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 10.0, 10.0)?;
    let drill = make_cylinder_brep(p(10.0, 5.0, -1.0), p(0.0, 0.0, 1.0), 3.0, 12.0)?;
    let drilled = cut(&block, &drill)?;
    let wall = face_at(&drilled, p(13.0, 5.0, 5.0))?;
    Ok(delete_faces_and_heal(&drilled, &[wall])?)
}

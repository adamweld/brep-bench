//! The box from the origin to (20, 10, 10) with the cylinder of radius 3 about the vertical line
//! through (10, 0) cut away: a half-round notch in the side y = 0. Then the notch's wall deleted,
//! the side, the top and the bottom closing over it: the box.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 10.0, 10.0)?;
    let cutter = make_cylinder_brep(p(10.0, 0.0, -1.0), p(0.0, 0.0, 1.0), 3.0, 12.0)?;
    let notched = cut(&block, &cutter)?;
    // the wall 60° round from the side, clear of the seam at its crown
    let wall = face_at(&notched, p(11.5, 1.5 * 3f64.sqrt(), 5.0))?;
    Ok(delete_faces_and_heal(&notched, &[wall])?)
}

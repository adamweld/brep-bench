//! A blind bore's beveled floor deleted: the floor and the wall meet again.
//!
//! The bevel is made as the case says, a cone face from joining or cutting frustums; then the kernel's
//! `delete_faces_and_heal` deletes it.
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use crate::solids::boolean::{cut, union};
use brep_kernel::{delete_faces_and_heal, make_box_brep, make_cone_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let block = make_box_brep(p(-10.0, -10.0, 0.0), 20.0, 20.0, 10.0)?;
    let cutter = union(&make_cylinder_brep(p(0.0, 0.0, 5.0), z, 4.0, 6.0)?, &make_cone_brep(p(0.0, 0.0, 4.0), z, 3.0, 4.0, 1.0)?)?;
    let beveled = cut(&block, &cutter)?;
    // the bevel, halfway along its slant
    let bevel = face_at(&beveled, p(0.0, -3.5, 4.5))?;
    Ok(delete_faces_and_heal(&beveled, &[bevel])?)
}

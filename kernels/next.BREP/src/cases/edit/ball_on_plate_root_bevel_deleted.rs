//! A ball standing in a plate, its beveled root deleted: the plate and the ball meet again.
//!
//! The bevel is made as the case says, a cone face from joining or cutting frustums; then the kernel's
//! `delete_faces_and_heal` deletes it.
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use crate::solids::boolean::union;
use brep_kernel::{delete_faces_and_heal, make_cone_brep, make_cylinder_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let plate = make_cylinder_brep(p(0.0, 0.0, -2.0), z, 8.0, 2.0)?;
    let joined = union(&plate, &make_sphere_brep(p(0.0, 0.0, 1.0), 3.0, z)?)?;
    let beveled = union(&joined, &make_cone_brep(p(0.0, 0.0, 0.0), z, 4.0, 3.0, 1.0)?)?;
    // the bevel, halfway along its slant
    let bevel = face_at(&beveled, p(0.0, -3.5, 0.5))?;
    Ok(delete_faces_and_heal(&beveled, &[bevel])?)
}

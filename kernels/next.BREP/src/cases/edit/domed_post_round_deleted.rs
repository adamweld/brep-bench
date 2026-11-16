//! A post capped by a ball: the cylinder r 4 (z 0..6) under the cap above z 6 of the ball r 5 about (0, 0, 3), its rim rounded 1;
//! then the round deleted: the domed post.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal`.
use crate::solids::boolean::{common, union};
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_box_brep, make_cylinder_brep, make_sphere_brep, BrepSolid};
pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let cap = common(&make_sphere_brep(p(0.0, 0.0, 3.0), 5.0, z)?, &make_box_brep(p(-6.0, -6.0, 6.0), 12.0, 12.0, 4.0)?)?;
    let b = union(&make_cylinder_brep(p(0.0, 0.0, 0.0), z, 4.0, 6.0)?, &cap)?;
    let r = fillet_edges(&b, &[p(0.0, -4.0, 6.0)], None, 1.0, false, None)?;
    let f = face_at(&r, p(0.0, -3.9426028923173684, 5.979667149854743))?;
    Ok(delete_faces_and_heal(&r, &[f])?)
}

//! A turned post's rounded shoulder deleted: the cylinder r 5 (z 0..6) under the frustum 5 -> 2 (z 6..10), its shoulder rounded 1;
//! then the round deleted: the post.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal`.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_cone_brep, make_cylinder_brep, BrepSolid};
pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let b = union(&make_cylinder_brep(p(0.0, 0.0, 0.0), z, 5.0, 6.0)?, &make_cone_brep(p(0.0, 0.0, 6.0), z, 5.0, 2.0, 4.0)?)?;
    let r = fillet_edges(&b, &[p(0.0, -5.0, 6.0)], None, 1.0, false, None)?;
    let f = face_at(&r, p(0.0, -4.948683298050514, 5.982894432683505))?;
    Ok(delete_faces_and_heal(&r, &[f])?)
}

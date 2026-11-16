//! A turned post's rounded shoulder deleted, its cone made turned a radian about the axis: the cylinder and the cone meet again.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal`.
use crate::solids::boolean::{turned_about_z, union};
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_cone_brep, make_cylinder_brep, BrepSolid};
pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let b = union(&make_cylinder_brep(p(0.0, 0.0, 0.0), z, 5.0, 6.0)?, &turned_about_z(&make_cone_brep(p(0.0, 0.0, 6.0), z, 5.0, 2.0, 4.0)?, 1.0)?)?;
    let r = fillet_edges(&b, &[p(-2.3971276930210146, 4.387912809451864, 6.0), p(2.397127693021014, -4.387912809451864, 6.0)], None, 1.0, false, None)?;
    let f = [face_at(&r, p(-2.372525155549491, 4.342878166687269, 5.982894432683505))?, face_at(&r, p(2.3725251555494906, -4.342878166687269, 5.982894432683505))?];
    Ok(delete_faces_and_heal(&r, &f)?)
}

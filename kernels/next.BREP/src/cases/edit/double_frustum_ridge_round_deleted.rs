//! Two cone frustums base to base (3 -> 5 over z 0..4, 5 -> 2 over z 4..8), their ridge rounded 1;
//! then the round deleted: the two frustums.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal`.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_cone_brep, BrepSolid};
pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let b = union(&make_cone_brep(p(0.0, 0.0, 0.0), z, 3.0, 5.0, 4.0)?, &make_cone_brep(p(0.0, 0.0, 4.0), z, 5.0, 2.0, 4.0)?)?;
    let r = fillet_edges(&b, &[p(0.0, -5.0, 4.0)], None, 1.0, false, None)?;
    let f = face_at(&r, p(0.0, -4.825138920703175, 3.984232786315833))?;
    Ok(delete_faces_and_heal(&r, &[f])?)
}

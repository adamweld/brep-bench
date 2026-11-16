//! Two cone frustums base to base, the upper made turned two radians about the axis, their rounded ridge deleted: the two cones meet again.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal`.
use crate::solids::boolean::{turned_about_z, union};
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_cone_brep, BrepSolid};
pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let b = union(&make_cone_brep(p(0.0, 0.0, 0.0), z, 3.0, 5.0, 4.0)?, &turned_about_z(&make_cone_brep(p(0.0, 0.0, 4.0), z, 5.0, 2.0, 4.0)?, 2.0)?)?;
    let r = fillet_edges(&b, &[p(-4.207354924039483, 2.701511529340699, 4.0), p(4.207354924039482, -2.7015115293406993, 4.0)], None, 1.0, false, None)?;
    let f = [face_at(&r, p(-4.060214399439012, 2.6070336849900326, 3.984232786315833))?, face_at(&r, p(4.060214399439011, -2.607033684990033, 3.984232786315833))?];
    Ok(delete_faces_and_heal(&r, &f)?)
}

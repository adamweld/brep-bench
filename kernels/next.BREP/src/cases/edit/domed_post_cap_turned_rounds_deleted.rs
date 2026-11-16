//! A post capped by a ball, the cap made turned 1.5 radians about the axis, its rounded rim deleted: the cylinder and the ball meet again.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal`.
use crate::solids::boolean::{common, turned_about_z, union};
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_box_brep, make_cylinder_brep, make_sphere_brep, BrepSolid};
pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let cap = common(&make_sphere_brep(p(0.0, 0.0, 3.0), 5.0, z)?, &make_box_brep(p(-6.0, -6.0, 6.0), 12.0, 12.0, 4.0)?)?;
    let b = union(&make_cylinder_brep(p(0.0, 0.0, 0.0), z, 4.0, 6.0)?, &turned_about_z(&cap, 1.5)?)?;
    let r = fillet_edges(&b, &[p(-2.7265550400933365, 2.9267554754952836, 6.0), p(2.726555040093336, -2.926755475495284, 6.0)], None, 1.0, false, None)?;
    let f = [face_at(&r, p(-2.6874309467836217, 2.88475865069835, 5.979667149854743))?, face_at(&r, p(2.6874309467836213, -2.8847586506983505, 5.979667149854743))?];
    Ok(delete_faces_and_heal(&r, &f)?)
}

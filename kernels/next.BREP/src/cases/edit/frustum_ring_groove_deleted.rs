//! A cone frustum about the z axis, radius 6 at z = 0 to 3 at z = 20, with a ring groove turned
//! into it (everything outside radius 4 from z = 8 to z = 12 cut away), the groove's floor and both
//! walls deleted together: the frustum's cone below and above the groove is one surface, which runs
//! on across it.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, make_cone_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let body = make_cone_brep(p(0.0, 0.0, 0.0), z, 6.0, 3.0, 20.0)?;
    // The ring from radius 4 to 7, z = 8 to 12.
    let ring = cut(&make_cylinder_brep(p(0.0, 0.0, 8.0), z, 7.0, 4.0)?, &make_cylinder_brep(p(0.0, 0.0, 7.0), z, 4.0, 6.0)?)?;
    let grooved = cut(&body, &ring)?;
    // The floor, and the walls at z = 8 and z = 12.
    let groove = [p(0.0, -4.0, 10.0), p(0.0, -4.5, 8.0), p(0.0, -4.1, 12.0)]
        .iter()
        .map(|point| face_at(&grooved, *point))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(delete_faces_and_heal(&grooved, &groove)?)
}

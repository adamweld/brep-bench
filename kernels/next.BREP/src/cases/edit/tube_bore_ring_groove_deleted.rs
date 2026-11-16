//! A tube about the z axis from z = 0 to z = 20, outer radius 10, bore radius 4, with a ring groove
//! turned into the bore (everything inside radius 6 from z = 8 to z = 12 cut away), the groove's
//! floor and both walls deleted together: the bore below and above the groove is one surface, which
//! runs on across it.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let tube = cut(&make_cylinder_brep(p(0.0, 0.0, 0.0), z, 10.0, 20.0)?, &make_cylinder_brep(p(0.0, 0.0, -1.0), z, 4.0, 22.0)?)?;
    let grooved = cut(&tube, &make_cylinder_brep(p(0.0, 0.0, 8.0), z, 6.0, 4.0)?)?;
    // The floor, and the walls at z = 8 and z = 12.
    let groove = [p(0.0, -6.0, 10.0), p(0.0, -5.0, 8.0), p(0.0, -5.0, 12.0)]
        .iter()
        .map(|point| face_at(&grooved, *point))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(delete_faces_and_heal(&grooved, &groove)?)
}

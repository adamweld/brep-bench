//! A hub (the tube about the z axis from z = 0 to 20, outer radius 10, bore 4) with a keyway along
//! its bore (the box −1 ≤ x ≤ 1, −5 ≤ y ≤ 0 cut away), the keyway's floor and walls deleted: the
//! bore closes over it.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let hub = cut(&make_cylinder_brep(p(0.0, 0.0, 0.0), z, 10.0, 20.0)?, &make_cylinder_brep(p(0.0, 0.0, -1.0), z, 4.0, 22.0)?)?;
    let body = cut(&hub, &make_box_brep(p(-1.0, -5.0, -1.0), 2.0, 5.0, 22.0)?)?;
    // The floor, and the walls x = 1 and x = −1.
    let faces = [p(0.0, -5.0, 10.0), p(1.0, -4.5, 10.0), p(-1.0, -4.5, 10.0)]
        .iter()
        .map(|point| face_at(&body, *point))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(delete_faces_and_heal(&body, &faces)?)
}

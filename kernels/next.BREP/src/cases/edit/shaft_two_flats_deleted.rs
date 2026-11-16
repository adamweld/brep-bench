//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with two opposite flats
//! (everything with |x| > 4 cut away), both flats deleted together: the cylinder closes over them.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let shaft = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let one = cut(&shaft, &make_box_brep(p(4.0, -6.0, -1.0), 2.0, 12.0, 22.0)?)?;
    let body = cut(&one, &make_box_brep(p(-6.0, -6.0, -1.0), 2.0, 12.0, 22.0)?)?;
    let faces = [p(4.0, 0.0, 10.0), p(-4.0, 0.0, 10.0)]
        .iter()
        .map(|point| face_at(&body, *point))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(delete_faces_and_heal(&body, &faces)?)
}

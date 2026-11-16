//! A ball of radius 5 about the origin, its top cut flat at z = 3, the flat deleted: the ball closes
//! over it.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, make_box_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let body = cut(&ball, &make_box_brep(p(-6.0, -6.0, 3.0), 12.0, 12.0, 3.0)?)?;
    let faces = [p(1.0, 0.0, 3.0)]
        .iter()
        .map(|point| face_at(&body, *point))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(delete_faces_and_heal(&body, &faces)?)
}

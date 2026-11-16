//! A cone frustum about the z axis, radius 6 at z = 0 to 3 at z = 20, with a flat milled along its
//! side (everything with y < −4 cut away), the flat deleted: the cone closes over it.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, make_box_brep, make_cone_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let frustum = make_cone_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 3.0, 20.0)?;
    let body = cut(&frustum, &make_box_brep(p(-7.0, -7.0, -1.0), 14.0, 3.0, 22.0)?)?;
    let faces = [p(0.0, -4.0, 5.0)]
        .iter()
        .map(|point| face_at(&body, *point))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(delete_faces_and_heal(&body, &faces)?)
}

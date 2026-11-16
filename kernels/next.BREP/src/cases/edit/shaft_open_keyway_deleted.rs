//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with an open keyway (the
//! box −1 ≤ x ≤ 1, y ≤ −3, z ≥ 8 cut away: 2 wide, from z = 8 out through the top end), its floor,
//! two walls and end deleted together: the shaft's cylinder and top close over it.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let shaft = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let body = cut(&shaft, &make_box_brep(p(-1.0, -6.0, 8.0), 2.0, 3.0, 13.0)?)?;
    let faces = [p(0.0, -3.0, 15.0), p(1.0, -4.0, 15.0), p(-1.0, -4.0, 15.0), p(0.0, -4.0, 8.0)]
        .iter()
        .map(|point| face_at(&body, *point))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(delete_faces_and_heal(&body, &faces)?)
}

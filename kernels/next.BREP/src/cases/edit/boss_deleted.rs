//! The cylinder of radius 8 on the z axis from z = −4 to z = 0 with the cylinder of radius 3 from z
//! = 0 to z = 8 standing on it; then the boss's side and top deleted, the plate's top closing over
//! its foot: the plate.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let plate = make_cylinder_brep(p(0.0, 0.0, -4.0), z, 8.0, 4.0)?;
    let boss = make_cylinder_brep(p(0.0, 0.0, 0.0), z, 3.0, 8.0)?;
    let solid = union(&plate, &boss)?;
    let (side, top) = (face_at(&solid, p(0.0, -3.0, 4.0))?, face_at(&solid, p(0.0, 0.0, 8.0))?);
    Ok(delete_faces_and_heal(&solid, &[side, top])?)
}

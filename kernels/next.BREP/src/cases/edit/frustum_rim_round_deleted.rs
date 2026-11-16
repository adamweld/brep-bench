//! The frustum from radius 6 at z = 0 to radius 3 at z = 6, its top rim rounded to radius 1; then
//! the round deleted, the top and the cone run on to meet in the rim again: the frustum.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the face.
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_cone_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let frustum = make_cone_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 3.0, 6.0)?;
    let rounded = fillet_edges(&frustum, &[p(0.0, -3.0, 6.0)], None, 1.0, false, None)?;
    // the round halfway round its tube, toward the old rim
    let round = face_at(&rounded, p(0.0, -2.9076971233692386, 5.850650808352039))?;
    Ok(delete_faces_and_heal(&rounded, &[round])?)
}

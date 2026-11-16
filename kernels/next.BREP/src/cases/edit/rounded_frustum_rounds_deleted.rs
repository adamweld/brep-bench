//! A cone frustum's two rounded rims deleted: its ends and its cone meet again: the frustum from radius 6 at z = 0 to radius 3 at z = 6, both rims rounded to radius 3/2;
//! then delete both rounds together.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal`.
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_cone_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let frustum = make_cone_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 3.0, 6.0)?;
    let rounded = fillet_edges(&frustum, &[p(0.0, -3.0, 6.0), p(0.0, -6.0, 0.0)], None, 1.5, false, None)?;
    // each round deleted, halfway round its tube toward its old rim
    let rounds = [
        face_at(&rounded, p(0.0, -2.861545685053858, 5.77597621252806))?,
        face_at(&rounded, p(0.0, -4.848925229403217, 0.7114033318212998))?,
    ];
    Ok(delete_faces_and_heal(&rounded, &rounds)?)
}

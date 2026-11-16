//! The cylinder of radius 6 on the z axis from z = 0 to z = 10, both its rims rounded to radius 2;
//! then both rounds, quarter tori, deleted together, the ends and the side run on to meet in the
//! rims again: the cylinder.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 10.0)?;
    let rims = [p(0.0, -6.0, 10.0), p(0.0, -6.0, 0.0)];
    let rounded = fillet_edges(&cylinder, &rims, None, 2.0, false, None)?;
    // each round at 45° round its tube, whose center line is the circle of radius 4 at z = 8 or 2
    let s = 2f64.sqrt();
    let rounds = [
        face_at(&rounded, p(0.0, -(4.0 + s), 8.0 + s))?,
        face_at(&rounded, p(0.0, -(4.0 + s), 2.0 - s))?,
    ];
    Ok(delete_faces_and_heal(&rounded, &rounds)?)
}

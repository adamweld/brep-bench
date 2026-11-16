//! The box from the origin to (20, 10, 10), the two long edges of its top rounded to radius 2; then
//! both rounds, quarter cylinders, deleted together, the top and the two sides run on to meet in
//! their edges again: the box.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 10.0, 10.0)?;
    let edges = [p(10.0, 0.0, 10.0), p(10.0, 10.0, 10.0)];
    let rounded = fillet_edges(&block, &edges, None, 2.0, false, None)?;
    // each round at its middle, 45° round its axis (y = 2 or y = 8, z = 8)
    let s = 2f64.sqrt();
    let rounds = [
        face_at(&rounded, p(10.0, 2.0 - s, 8.0 + s))?,
        face_at(&rounded, p(10.0, 8.0 + s, 8.0 + s))?,
    ];
    Ok(delete_faces_and_heal(&rounded, &rounds)?)
}

//! The box from the origin to (20, 10, 10) drilled through by a hole of radius 2 about the line
//! x = 10, y = 5, both of the hole's rims rounded to radius 1; then both rounds deleted together:
//! the drilled block.
//!
//! The hole is a cylinder cut from the box; then the kernel's `delete_faces_and_heal` deletes the
//! faces.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{
    delete_faces_and_heal, fillet_edges, make_box_brep, make_cylinder_brep, BrepSolid,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 10.0, 10.0)?;
    let drilled = cut(&block, &make_cylinder_brep(p(10.0, 5.0, -1.0), p(0.0, 0.0, 1.0), 2.0, 12.0)?)?;
    let rims = [p(10.0, 3.0, 10.0), p(10.0, 3.0, 0.0)];
    let rounded = fillet_edges(&drilled, &rims, None, 1.0, false, None)?;
    // each round at 45° round its tube, about the circle of radius 3 at z = 9 or 1
    let s = 2f64.sqrt() / 2.0;
    let rounds = [
        face_at(&rounded, p(10.0, 5.0 - (3.0 - s), 9.0 + s))?,
        face_at(&rounded, p(10.0, 5.0 - (3.0 - s), 1.0 - s))?,
    ];
    Ok(delete_faces_and_heal(&rounded, &rounds)?)
}

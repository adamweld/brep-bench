//! The L outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15) extruded 8 along +z;
//! then its riser x = 5 and its tread y = 5 deleted together, the far end y = 15 and the far side x
//! = 20 run on to meet: the box from the origin to (20, 15, 8).
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::{p, polygon};
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, extrude_profile_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let outline = polygon(&[
        p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0), p(20.0, 5.0, 0.0),
        p(5.0, 5.0, 0.0), p(5.0, 15.0, 0.0), p(0.0, 15.0, 0.0),
    ])?;
    let block = extrude_profile_brep(&outline, p(0.0, 0.0, 1.0), 8.0)?;
    let (riser, tread) = (face_at(&block, p(5.0, 10.0, 4.0))?, face_at(&block, p(12.0, 5.0, 4.0))?);
    Ok(delete_faces_and_heal(&block, &[riser, tread])?)
}

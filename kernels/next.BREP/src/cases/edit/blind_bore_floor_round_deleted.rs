//! The box from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 4 down to z = 4, the
//! floor's edge rounded to radius 1; then the round deleted, the floor and the wall run on to meet
//! again: the sharp-floored bore.
//!
//! The bore is a cylinder cut from the box; then the kernel's `delete_faces_and_heal` deletes the
//! round.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{
    delete_faces_and_heal, fillet_edges, make_box_brep, make_cylinder_brep, BrepSolid,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(-10.0, -10.0, 0.0), 20.0, 20.0, 10.0)?;
    let bored = cut(&block, &make_cylinder_brep(p(0.0, 0.0, 4.0), p(0.0, 0.0, 1.0), 4.0, 7.0)?)?;
    let rounded = fillet_edges(&bored, &[p(0.0, -4.0, 4.0)], None, 1.0, false, None)?;
    // the round halfway round its tube, toward the old edge
    let round = face_at(&rounded, p(0.0, -3.7071067811865475, 4.292893218813452))?;
    Ok(delete_faces_and_heal(&rounded, &[round])?)
}

//! The half of the ball of radius 5 above z = 0, its rim rounded to radius 1; then the round
//! deleted, the flat and the ball run on to meet in the rim again: the hemisphere.
//!
//! The half ball is the kernel's ball less the block under z = 0; then the kernel's
//! `delete_faces_and_heal` deletes the round.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_box_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let half = cut(&ball, &make_box_brep(p(-6.0, -6.0, -6.0), 12.0, 12.0, 6.0)?)?;
    let rounded = fillet_edges(&half, &[p(0.0, -5.0, 0.0)], None, 1.0, false, None)?;
    // the round halfway round its tube, toward the old rim
    let round = face_at(&rounded, p(0.0, -4.620983133994804, 0.3363010339995828))?;
    Ok(delete_faces_and_heal(&rounded, &[round])?)
}

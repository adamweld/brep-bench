//! A half torus's rounded outer rim deleted: the flat and the torus meet again: the half torus (tube radius 2 about the circle of radius 6, above z = 0), its outer rim
//! rounded to radius 1/2; then the round deleted: the half torus.
//!
//! The half torus is the kernel's torus in common with a box above z = 0; then the kernel's `delete_faces_and_heal`.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_box_brep, make_torus_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let torus = make_torus_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 2.0)?;
    let half = common(&torus, &make_box_brep(p(-10.0, -10.0, 0.0), 20.0, 20.0, 5.0)?)?;
    let rounded = fillet_edges(&half, &[p(0.0, -8.0, 0.0)], None, 0.5, false, None)?;
    // each round halfway round its tube, toward the old rim
    let rounds = [
        face_at(&rounded, p(0.0, -7.794515558701765, 0.1753919415842729))?,
    ];
    Ok(delete_faces_and_heal(&rounded, &rounds)?)
}

//! The plate of radius 8 from z = −2 to 0 joined to the ball of radius 3 about (0, 0, 1), their
//! concave edge rounded to radius 1; then the round deleted, the top and the ball run on to meet
//! again.
//!
//! The plate and the ball are joined, the edge filleted; then the kernel's `delete_faces_and_heal`
//! deletes the round.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{
    delete_faces_and_heal, fillet_edges, make_cylinder_brep, make_sphere_brep, BrepSolid,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let plate = make_cylinder_brep(p(0.0, 0.0, -2.0), z, 8.0, 2.0)?;
    let joined = union(&plate, &make_sphere_brep(p(0.0, 0.0, 1.0), 3.0, z)?)?;
    let rounded = fillet_edges(&joined, &[p(0.0, -(8f64).sqrt(), 0.0)], None, 1.0, false, None)?;
    // the round halfway round its tube, toward the old edge
    let round = face_at(&rounded, p(0.0, -3.2393960073426595, 0.3507838831685456))?;
    Ok(delete_faces_and_heal(&rounded, &[round])?)
}

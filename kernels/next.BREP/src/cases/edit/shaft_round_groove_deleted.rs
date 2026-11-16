//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a round-bottomed
//! groove turned into it (the torus whose tube, radius 1.5, runs round the circle of radius 5 at
//! z = 10, cut away), the groove's one face deleted: the shaft's cylinder below and above the groove
//! is one surface, which runs on across it.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, make_cylinder_brep, make_torus_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let shaft = make_cylinder_brep(p(0.0, 0.0, 0.0), z, 5.0, 20.0)?;
    let grooved = cut(&shaft, &make_torus_brep(p(0.0, 0.0, 10.0), z, 5.0, 1.5)?)?;
    // The groove's deepest circle, radius 3.5 at z = 10.
    let groove = [p(0.0, -3.5, 10.0)]
        .iter()
        .map(|point| face_at(&grooved, *point))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(delete_faces_and_heal(&grooved, &groove)?)
}

//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a V-groove turned
//! into it (everything farther from the axis than 3 + |z − 10| cut away: two cones meeting at radius
//! 3 at z = 10), both cones deleted together: the shaft's cylinder below and above the groove is one
//! surface, which runs on across it.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, make_cone_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let shaft = make_cylinder_brep(p(0.0, 0.0, 0.0), z, 5.0, 20.0)?;
    // Below z = 10, everything outside the cone from radius 6 at z = 7 to 3 at z = 10; above, its mirror.
    let lower = cut(&make_cylinder_brep(p(0.0, 0.0, 7.0), z, 7.0, 3.0)?, &make_cone_brep(p(0.0, 0.0, 7.0), z, 6.0, 3.0, 3.0)?)?;
    let upper = cut(&make_cylinder_brep(p(0.0, 0.0, 10.0), z, 7.0, 3.0)?, &make_cone_brep(p(0.0, 0.0, 10.0), z, 3.0, 6.0, 3.0)?)?;
    let grooved = cut(&cut(&shaft, &lower)?, &upper)?;
    // A point on each cone, radius 4 at z = 9 and z = 11.
    let groove = [p(0.0, -4.0, 9.0), p(0.0, -4.0, 11.0)]
        .iter()
        .map(|point| face_at(&grooved, *point))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(delete_faces_and_heal(&grooved, &groove)?)
}

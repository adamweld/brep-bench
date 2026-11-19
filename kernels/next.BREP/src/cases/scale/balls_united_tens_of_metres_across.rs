//! A ball of radius 50000 about the origin and a ball of radius 40000 about (20000, 30000, 60000),
//! 70000 from the first, united: two balls tens of metres across, if a unit is a millimetre.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = make_sphere_brep(p(0.0, 0.0, 0.0), 50000.0, p(0.0, 0.0, 1.0))?;
    let second = make_sphere_brep(p(20000.0, 30000.0, 60000.0), 40000.0, p(0.0, 0.0, 1.0))?;
    Ok(union(&first, &second)?)
}

//! A ball of radius 0.005 about the origin and a ball of radius 0.004 about
//! (0.002, 0.003, 0.006), 0.007 from the first, united: two balls a few microns across, if a unit
//! is a millimetre.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = make_sphere_brep(p(0.0, 0.0, 0.0), 0.005, p(0.0, 0.0, 1.0))?;
    let second = make_sphere_brep(p(0.002, 0.003, 0.006), 0.004, p(0.0, 0.0, 1.0))?;
    Ok(union(&first, &second)?)
}

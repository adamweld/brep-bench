//! A ball of radius 5 about (600000, −800000, 0) and a ball of radius 4 about
//! (600002, −799997, 6), 7 from the first along (2, 3, 6)/7, united: two balls a million from the
//! origin.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = make_sphere_brep(p(600000.0, -800000.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let second = make_sphere_brep(p(600002.0, -799997.0, 6.0), 4.0, p(0.0, 0.0, 1.0))?;
    Ok(union(&first, &second)?)
}

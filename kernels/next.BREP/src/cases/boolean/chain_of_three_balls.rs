//! Balls of radius 5, 4 and 3 about the origin, (7, 0, 0) and (10, 4, 0), united one after
//! another: the first two, then the third with that.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let second = make_sphere_brep(p(7.0, 0.0, 0.0), 4.0, p(0.0, 0.0, 1.0))?;
    let third = make_sphere_brep(p(10.0, 4.0, 0.0), 3.0, p(0.0, 0.0, 1.0))?;
    let pair = union(&first, &second)?;
    Ok(union(&pair, &third)?)
}

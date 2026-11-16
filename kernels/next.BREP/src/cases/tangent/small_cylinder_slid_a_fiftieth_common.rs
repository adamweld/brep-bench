//! A small cylinder, radius 0.00075 and height 0.0015 about the z axis from z = 0, and a copy of
//! it slid 0.000015 along x (a fiftieth of its radius): their common part.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

const RADIUS: f64 = 0.00075;
const HEIGHT: f64 = 0.0015;
const SLIDE: f64 = 0.000015;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let first = make_cylinder_brep(p(0.0, 0.0, 0.0), z, RADIUS, HEIGHT)?;
    let second = make_cylinder_brep(p(SLIDE, 0.0, 0.0), z, RADIUS, HEIGHT)?;
    Ok(common(&first, &second)?)
}

//! A cylinder of radius 5 and height 10 about the z axis from z = 0, less a cylinder of radius 4.999
//! on the same axis between the same two planes. The first less the second.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

const OUTER: f64 = 5.0;
const INNER: f64 = 4.999;
const HEIGHT: f64 = 10.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let outer = make_cylinder_brep(p(0.0, 0.0, 0.0), z, OUTER, HEIGHT)?;
    let inner = make_cylinder_brep(p(0.0, 0.0, 0.0), z, INNER, HEIGHT)?;
    Ok(cut(&outer, &inner)?)
}

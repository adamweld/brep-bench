//! A small cylinder (radius 0.00075, height 0.0015) and a copy a hundredth wider
//! (radius 0.0007575) on the same axis: the wider copy less it.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

const RADIUS: f64 = 0.00075;
const WIDER: f64 = 0.0007575;
const HEIGHT: f64 = 0.0015;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let first = make_cylinder_brep(p(0.0, 0.0, 0.0), z, WIDER, HEIGHT)?;
    let second = make_cylinder_brep(p(0.0, 0.0, 0.0), z, RADIUS, HEIGHT)?;
    Ok(cut(&first, &second)?)
}

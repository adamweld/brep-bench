//! A small cube of side 0.0015 from the origin, less a copy of itself raised by 0.000015 along z, a
//! hundredth of its side: the slab from the origin to (0.0015, 0.0015, 0.000015).
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

const SIDE: f64 = 0.0015;
const RISE: f64 = 0.000015;

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(0.0, 0.0, 0.0), SIDE, SIDE, SIDE)?;
    let raised = make_box_brep(p(0.0, 0.0, RISE), SIDE, SIDE, SIDE)?;
    Ok(cut(&cube, &raised)?)
}

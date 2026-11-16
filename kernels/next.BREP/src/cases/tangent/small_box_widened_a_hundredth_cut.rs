//! A small square prism (0.0015 square about the z axis, 0.0015 tall) and a copy a hundredth wider
//! (0.001515 square) between the same planes: the wider copy less it.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

const SIDE: f64 = 0.0015;
const WIDER: f64 = 0.001515;
const HEIGHT: f64 = 0.0015;

pub fn build() -> Result<BrepSolid, Refusal> {
    let wider = make_box_brep(p(-WIDER / 2.0, -WIDER / 2.0, 0.0), WIDER, WIDER, HEIGHT)?;
    let prism = make_box_brep(p(-SIDE / 2.0, -SIDE / 2.0, 0.0), SIDE, SIDE, HEIGHT)?;
    Ok(cut(&wider, &prism)?)
}

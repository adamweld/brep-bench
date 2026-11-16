//! A small square prism (0.0015 square about the z axis, 0.0015 tall) and a copy a hundredth wider
//! (0.001515 square) between the same planes: the wider copy less it.
//!
//! Both made as blocks; cadrs's boolean takes them.
use crate::bench::{block, cut, p3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const SIDE: f64 = 0.0015;
const WIDER: f64 = 0.001515;
const HEIGHT: f64 = 0.0015;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let wider = block(k, p3(-WIDER / 2.0, -WIDER / 2.0, 0.0), p3(WIDER / 2.0, WIDER / 2.0, HEIGHT))?;
    let prism = block(k, p3(-SIDE / 2.0, -SIDE / 2.0, 0.0), p3(SIDE / 2.0, SIDE / 2.0, HEIGHT))?;
    cut(k, wider, prism)
}

//! A small cylinder (radius 0.00075, height 0.0015) and a copy a hundredth wider
//! (radius 0.0007575) on the same axis: the wider copy less it.
//!
//! Both made as primitives; cadrs's boolean takes them.
use crate::bench::{cylinder, cut, p3, v3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const RADIUS: f64 = 0.00075;
const WIDER: f64 = 0.0007575;
const HEIGHT: f64 = 0.0015;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let first = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), WIDER, HEIGHT)?;
    let second = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), RADIUS, HEIGHT)?;
    cut(k, first, second)
}

//! A small cylinder, radius 0.00075 and height 0.0015 about the z axis from z = 0, and a copy of
//! it raised 0.000015 along z (a hundredth of its height): the first less the second.
//!
//! The second is the first moved; cadrs's boolean takes them.
use crate::bench::{cylinder, cut, moved, p3, v3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const RADIUS: f64 = 0.00075;
const HEIGHT: f64 = 0.0015;
const RISE: f64 = 0.000015;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let first = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), RADIUS, HEIGHT)?;
    let second = moved(k, first, v3(0.0, 0.0, RISE))?;
    cut(k, first, second)
}

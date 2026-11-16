//! A cylinder of radius 5 and height 10 about the z axis from z = 0, less a cylinder of radius 4.9999
//! on the same axis between the same two planes. The first less the second.
//!
//! Both made as primitives; cadrs's boolean takes them.
use crate::bench::{cut, cylinder, p3, v3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const OUTER: f64 = 5.0;
const INNER: f64 = 4.9999;
const HEIGHT: f64 = 10.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let outer = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), OUTER, HEIGHT)?;
    let inner = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), INNER, HEIGHT)?;
    cut(k, outer, inner)
}

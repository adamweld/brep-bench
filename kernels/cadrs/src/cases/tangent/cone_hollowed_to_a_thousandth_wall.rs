//! A cone, its base of radius 5 at z = 0 and its apex 10 above it on the z axis, less a cone of base
//! radius 4.999 with the same base plane and the same apex. The first less the second.
//!
//! Both made as primitives; cadrs's boolean takes them.
use crate::bench::{cone, cut, p3, v3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const OUTER: f64 = 5.0;
const INNER: f64 = 4.999;
const HEIGHT: f64 = 10.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let outer = cone(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), OUTER, 0.0, HEIGHT)?;
    let inner = cone(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), INNER, 0.0, HEIGHT)?;
    cut(k, outer, inner)
}

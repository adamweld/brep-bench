//! A small cube of side 0.001 from the origin and a copy of itself turned 1.2° about the z axis, the
//! line of one of its own edges, their common part: a prism over the quadrilateral the two squares
//! share.
//!
//! The cube is a square extruded; the copy is the cube turned about the z axis (cadrs's Transform).
use crate::bench::{block, common, p3, turned, v3};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel};

const SIDE: f64 = 0.001;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let cube = block(k, p3(0.0, 0.0, 0.0), p3(SIDE, SIDE, SIDE))?;
    let copy = turned(k, cube, Axis::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)), 1.2)?;
    common(k, cube, copy)
}

//! A small cube of side 0.0015 from the origin, united with a copy of itself raised by 0.000015 along
//! z, a hundredth of its side: the box from the origin to (0.0015, 0.0015, 0.001515).
//!
//! The cube is a square extruded; the copy is the cube moved up (cadrs's Transform).
use crate::bench::{block, union, moved, p3, v3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const SIDE: f64 = 0.0015;
const RISE: f64 = 0.000015;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let cube = block(k, p3(0.0, 0.0, 0.0), p3(SIDE, SIDE, SIDE))?;
    let raised = moved(k, cube, v3(0.0, 0.0, RISE))?;
    union(k, cube, raised)
}

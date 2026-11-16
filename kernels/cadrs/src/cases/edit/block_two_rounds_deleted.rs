//! The box from the origin to (20, 10, 10), the two long edges of its top rounded to radius 2; then
//! both rounds, quarter cylinders, deleted together, the top and the two sides run on to meet in
//! their edges again: the box.
//!
//! The block is filleted on both edges, radius 2; then Delete face takes the two rounds.
use crate::bench::{block, p3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(20.0, 10.0, 10.0))?;
    let front = k.edge_at(b, p3(10.0, 0.0, 10.0))?;
    let back = k.edge_at(b, p3(10.0, 10.0, 10.0))?;
    let rounded = fillet(k, b, &[front, back], 2.0)?;
    // each round at its middle, 45° round its axis (y = 2 or y = 8, z = 8)
    let s = 2f64.sqrt();
    let rounds = k.faces_at(rounded, &[p3(10.0, 2.0 - s, 8.0 + s), p3(10.0, 8.0 + s, 8.0 + s)])?;
    deleted(k, rounded, &rounds)
}

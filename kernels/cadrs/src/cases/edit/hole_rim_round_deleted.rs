//! The box from the origin to (20, 10, 10) drilled through by a hole of radius 2 about the line
//! x = 10, y = 5, the hole's top rim rounded to radius 1; then the round, a quarter torus, deleted,
//! the top and the hole's wall run on to meet in the rim again: the drilled block.
//!
//! The hole is a cylinder cut from the block, its rim filleted, radius 1; then Delete face takes
//! the round.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(20.0, 10.0, 10.0))?;
    let bore = cylinder(k, p3(10.0, 5.0, -1.0), v3(0.0, 0.0, 1.0), 2.0, 12.0)?;
    let drilled = cut(k, b, bore)?;
    let rim = k.edge_at(drilled, p3(10.0, 3.0, 10.0))?;
    let rounded = fillet(k, drilled, &[rim], 1.0)?;
    // the round at 45° round its tube, whose center line is the circle of radius 3 at z = 9
    let s = 2f64.sqrt() / 2.0;
    let round = k.faces_at(rounded, &[p3(10.0, 5.0 - (3.0 - s), 9.0 + s)])?;
    deleted(k, rounded, &round)
}

//! The box from the origin to (20, 10, 10) drilled through by a hole of radius 2 about the line
//! x = 10, y = 5, both of the hole's rims rounded to radius 1; then both rounds deleted together.
//! The hole a cylinder cut from the block, both rims filleted; then Delete face.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(20.0, 10.0, 10.0))?;
    let bore = cylinder(k, p3(10.0, 5.0, -1.0), v3(0.0, 0.0, 1.0), 2.0, 12.0)?;
    let drilled = cut(k, b, bore)?;
    let top = k.edge_at(drilled, p3(10.0, 3.0, 10.0))?;
    let bottom = k.edge_at(drilled, p3(10.0, 3.0, 0.0))?;
    let rounded = fillet(k, drilled, &[top, bottom], 1.0)?;
    // each round at 45° round its tube, about the circle of radius 3 at z = 9 or 1
    let s = 2f64.sqrt() / 2.0;
    let rounds = k.faces_at(rounded, &[p3(10.0, 5.0 - (3.0 - s), 9.0 + s), p3(10.0, 5.0 - (3.0 - s), 1.0 - s)])?;
    deleted(k, rounded, &rounds)
}

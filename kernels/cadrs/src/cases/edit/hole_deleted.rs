//! The box from the origin to (20, 10, 10), drilled through along z by a hole of radius 3 through
//! (10, 5); then the hole's wall deleted, the top and the bottom closing over its two ends: the
//! box.
//!
//! The block is drilled by a Boolean cut; then Delete face takes the hole's wall.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(20.0, 10.0, 10.0))?;
    let drill = cylinder(k, p3(10.0, 5.0, -1.0), v3(0.0, 0.0, 1.0), 3.0, 12.0)?;
    let drilled = cut(k, b, drill)?;
    let wall = k.faces_at(drilled, &[p3(13.0, 5.0, 5.0)])?;
    deleted(k, drilled, &wall)
}

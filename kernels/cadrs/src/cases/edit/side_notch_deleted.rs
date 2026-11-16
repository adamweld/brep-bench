//! The box from the origin to (20, 10, 10) with the cylinder of radius 3 about the vertical line
//! through (10, 0) cut away: a half-round notch in the side y = 0. Then the notch's wall deleted,
//! the side, the top and the bottom closing over it: the box.
//!
//! The notch is a Boolean cut; then Delete face takes its wall.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(20.0, 10.0, 10.0))?;
    let cutter = cylinder(k, p3(10.0, 0.0, -1.0), v3(0.0, 0.0, 1.0), 3.0, 12.0)?;
    let notched = cut(k, b, cutter)?;
    let wall = k.faces_at(notched, &[p3(10.0, 3.0, 5.0)])?;
    deleted(k, notched, &wall)
}

//! The box from the origin to (20, 10, 10) with a slot in its side y = 0: 4 wide from x = 8 to x =
//! 12, in to y = 5, closed by a half cylinder of radius 2 about the vertical line through (10, 5).
//! Then the slot's two flats and its half cylinder deleted, the side, the top and the bottom
//! closing over it: the box.
//!
//! The slot is two Boolean cuts; then Delete face takes its three faces.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(20.0, 10.0, 10.0))?;
    // the slot's straight part, run on past the side, the top and the bottom, and its round end
    let straight = block(k, p3(8.0, -1.0, -1.0), p3(12.0, 5.0, 11.0))?;
    let round = cylinder(k, p3(10.0, 5.0, -1.0), v3(0.0, 0.0, 1.0), 2.0, 12.0)?;
    let open = cut(k, b, straight)?;
    let slotted = cut(k, open, round)?;
    let faces = k.faces_at(slotted, &[p3(8.0, 2.0, 5.0), p3(12.0, 2.0, 5.0), p3(10.0, 7.0, 5.0)])?;
    deleted(k, slotted, &faces)
}

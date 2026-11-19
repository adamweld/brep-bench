//! A block, the box from (0, −5, 0) to (20, 5, 10), with a slot cut across its top by an end mill,
//! the solid cylinder of radius 2 about the vertical line through (−5, 0) from z = 7 to z = 15,
//! slid without turning along x by 30.
//!
//! solidcraft-kernel has no sweep of a solid, so the mill's sweep is built by elementary geometry.
//! Split by the plane x = −5 through its axis, square to the slide, the mill's back half leads
//! with its flat cut face only (its curved side faces back, and its ends move in their own
//! planes), so it sweeps itself and that rectangle slid 30; the front half likewise sweeps the
//! rectangle and itself where it stops. So the sweep is the back half where the mill starts, the
//! box from (−5, −2, 7) to (25, 2, 15) and the front half where it stops; the halves lie wholly
//! before x = −5 and past x = 25, clear of the block, so the box is what the sweep takes from it.
use crate::solids::*;

pub fn build() -> R<Body> {
    let stock = block((0.0, -5.0, 0.0), (20.0, 5.0, 10.0))?;
    cut(&stock, &block((-5.0, -2.0, 7.0), (25.0, 2.0, 15.0))?)
}

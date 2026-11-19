//! A plate, the box from (0, −5, 0) to (20, 5, 4), with a slot punched through it by a drum, the
//! solid cylinder of radius 2 about the line y = 0, z = −5 from x = 6 to x = 14, slid without
//! turning straight up by 16.
//!
//! solidcraft-kernel has no sweep of a solid, so the drum's sweep is built by elementary geometry.
//! Split by the plane z = −5 through its axis, the drum's lower half leads with its flat top only
//! (its curved side faces down, away from the slide, and its ends move in their own planes), so
//! it sweeps itself and that rectangle slid up 16; the upper half likewise sweeps the rectangle
//! and itself where it stops. So the sweep is the lower half where the drum starts, the box from
//! (6, −2, −5) to (14, 2, 11) and the upper half where it stops; the halves lie wholly below
//! z = −5 and above z = 11, clear of the plate, so the box is what the sweep takes from it.
use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((0.0, -5.0, 0.0), (20.0, 5.0, 4.0))?;
    cut(&plate, &block((6.0, -2.0, -5.0), (14.0, 2.0, 11.0))?)
}

//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a thread cut
//! partway up it by the bar 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1, carried along the screw about the
//! z axis that rises 6 a turn from a turn of −90° to one of 900°, then slid along −x by 3, straight
//! out of its groove.
//!
//! solidcraft-kernel has no sweep of a solid, so the bar's sweep is the union of its two motions'
//! sweeps. The screw's is built as `square-bar-screwed-through-rod`'s is: the bar where it
//! starts, wholly below the rod, and where the screw stops, with the screw sweeps of its section
//! by y = 0, its faces z = ±1 and its back, 12 from the axis. Where the screw stops the bar is
//! −12 ≤ x ≤ −4, −8 ≤ y ≤ 8, 14 ≤ z ≤ 16, and slid along its own length by 3 it passes over the
//! box −15 ≤ x ≤ −4 of the same section, which holds the bar where the screw stops as well.
use super::square_bar_screwed_through_rod::bar_cut;
use crate::solids::*;

pub fn build() -> R<Body> {
    let rod = zcyl(0.0, 0.0, 5.0, 0.0, 20.0)?;
    let threaded = bar_cut(rod, (4.0, -8.0, -1.0), (12.0, 8.0, 1.0), 6.0, -90.0, 900.0)?;
    cut(&threaded, &block((-15.0, -8.0, 14.0), (-4.0, 8.0, 16.0))?)
}

//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, threaded by the bar
//! 4.5 ≤ x ≤ 12, −8 ≤ y ≤ 8, −0.45 ≤ z ≤ 0.45 carried along the screw about the z axis that
//! rises 1 a turn, from a turn of −180° to one of 7380°, so fine that the groove's turns meet.
//!
//! solidcraft-kernel has no sweep of a solid, so the bar's sweep is built as
//! `square-bar-screwed-through-rod`'s is: the bar at its ends, below and above the rod, with the
//! screw sweeps of its section by y = 0, its faces z = ±0.45 and its back, 12 from the axis. Each
//! of the three sweeps clears itself from turn to turn (the section is 0.9 of the lead); where
//! the groove's turns meet, their sweeps overlap.
use super::square_bar_screwed_through_rod::bar_cut;
use crate::solids::*;

pub fn build() -> R<Body> {
    let rod = zcyl(0.0, 0.0, 5.0, 0.0, 20.0)?;
    bar_cut(rod, (4.5, -8.0, -0.45), (12.0, 8.0, 0.45), 1.0, -180.0, 7380.0)
}

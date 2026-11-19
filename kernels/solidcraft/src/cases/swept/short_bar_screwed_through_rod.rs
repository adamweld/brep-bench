//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, threaded by the short
//! bar 4 ≤ x ≤ 12, −1.5 ≤ y ≤ 1.5, −1 ≤ z ≤ 1 carried along the screw about the z axis that rises
//! 6 a turn, from a turn of −90° to one of 1350°.
//!
//! solidcraft-kernel has no sweep of a solid, so the bar's sweep is built as
//! `square-bar-screwed-through-rod`'s is: split by the plane y = 0, the half on y ≥ 0 leaves a
//! point only through its face z = −1, its back or the cut, and the half on y ≤ 0 first reaches
//! one only through its face z = 1, its back or the cut (its sides y = ±1.5 lead and trail as the
//! long bar's do). So the sweep is the bar at its ends, below and above the rod, with the screw
//! sweeps of its section by y = 0, its faces z = ±1 and its back, 12 from the axis.
use super::square_bar_screwed_through_rod::bar_cut;
use crate::solids::*;

pub fn build() -> R<Body> {
    let rod = zcyl(0.0, 0.0, 5.0, 0.0, 20.0)?;
    bar_cut(rod, (4.0, -1.5, -1.0), (12.0, 1.5, 1.0), 6.0, -90.0, 1350.0)
}

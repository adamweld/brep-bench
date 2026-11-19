//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a thread cut
//! partway up it by the bar 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1 carried along the screw about the
//! z axis that rises 6 a turn, from a turn of −120° to one of 600°, where it stops inside the rod.
//!
//! solidcraft-kernel has no sweep of a solid, so the bar's sweep is built as
//! `square-bar-screwed-through-rod`'s is: the bar where it starts and where it ends, with the
//! screw sweeps of its section by y = 0, its faces z = ±1 and its back, 12 from the axis. It
//! starts wholly below the rod, but ends inside it, so the bar where it stops is cut as well.
use super::square_bar_screwed_through_rod::{bar_cut, screwed};
use crate::solids::*;

pub fn build() -> R<Body> {
    let rod = zcyl(0.0, 0.0, 5.0, 0.0, 20.0)?;
    let grooved = bar_cut(rod, (4.0, -8.0, -1.0), (12.0, 8.0, 1.0), 6.0, -120.0, 600.0)?;
    // the bar where it stops: its face z = 1 carried to the turn of 600°, extruded down by 2
    let top = Plane { origin: v3(0.0, 0.0, 1.0), x: Vec3::X, y: Vec3::Y };
    let stop = extrude(&screwed(&top, 6.0, 600.0), rect(4.0, -8.0, 12.0, 8.0), -2.0, 0.0)?;
    cut(&grooved, &stop)
}

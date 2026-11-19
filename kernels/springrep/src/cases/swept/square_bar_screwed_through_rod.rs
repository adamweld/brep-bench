//! The rod of radius 5 about the z axis from z = 0 to z = 20, less everything the bar
//! 4 ≤ x ≤ 12, |y| ≤ 8, |z| ≤ 1 passes through as it is screwed about the z axis, rising 6 a turn,
//! from a turn of −90° to one of 1350°: a square thread.
//!
//! springrep has no sweep of a solid. The bar is 2 thick along the screw's axis, which the screw
//! rises in a turn of 120°: at each height of the rod the bar passes wholly, staying there through
//! 120°, and what it covers there is its level section turned through 120°, turned on with the
//! height — a twisted extrusion (`shared::swept::screwed_bar`). Only the bar within 5.5 of the
//! axis reaches the rod.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, rod, screwed_bar, turned_bar_section};

pub fn build() -> Result<Brep, String> {
    let delta = 2.0 * PI * 2.0 / 6.0;
    let section = turned_bar_section(4.0, None, 5.5, delta);
    let groove = screwed_bar(&section, 2.0, 6.0, deg(-90.0), deg(1350.0), delta)?;
    difference(&rod()?, &groove).map_err(e)
}

//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything the drum of
//! radius 1 about the upright line through (10, 0), from z = −0.5 to 0.5, passes through as it
//! turns about the z axis from −140° to 30°.
//!
//! springrep has no sweep of a solid. Every circle about the z axis meets the drum in one arc,
//! across the plane through both axes, so the sweep is the drum where it starts and where it stops
//! and its section by that plane, the rectangle 9 ≤ ρ ≤ 11, |z| ≤ 0.5, turned through the 170°
//! (`shared::swept`, a turn).
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::swept::{deg, rect, sector_block, swing, turned, unite};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let drum = cylinder(&at([10.0, 0.0, -0.5]), 1.0, 1.0, "drum").map_err(e)?;
    let (from, to) = (deg(-140.0), deg(30.0));
    let swept = swing(&rect(9.0, -0.5, 11.0, 0.5), from, to - from, "notch")?;
    let notch = unite(&[&turned(&drum, from), &swept, &turned(&drum, to)])?;
    difference(&sector_block()?, &notch).map_err(e)
}

//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything half a drum
//! passes through as it turns about the z axis from −140° to 30°: the half with y ≤ 0 of the drum
//! of radius 1 about the upright line through (10, 0), from z = −0.5 to 0.5, flat face first.
//!
//! springrep has no sweep of a solid. Every circle about the z axis meets the half drum in one arc
//! ending on its flat face, in the plane y = 0 through the z axis, so the sweep is the half drum
//! where it starts and where it stops and that face, the rectangle 9 ≤ ρ ≤ 11, |z| ≤ 0.5, turned
//! through the 170° (`shared::swept`, a turn).
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::swept::{arc, deg, line, outline, rect, sector_block, swing, turned, unite};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let half = outline(vec![
        arc((10.0, 0.0), 1.0, PI, PI, "side"),
        line((11.0, 0.0), (9.0, 0.0), "face"),
    ]);
    let tool = extrude(&half, &at([0.0, 0.0, -0.5]), 1.0, "half_drum").map_err(e)?;
    let (from, to) = (deg(-140.0), deg(30.0));
    let swept = swing(&rect(9.0, -0.5, 11.0, 0.5), from, to - from, "notch")?;
    let notch = unite(&[&turned(&tool, from), &swept, &turned(&tool, to)])?;
    difference(&sector_block()?, &notch).map_err(e)
}

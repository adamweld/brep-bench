//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything the bar
//! 10 ≤ x ≤ 11, |y| ≤ 0.5, |z| ≤ 0.6 passes through as it turns about the z axis from −140° to 30°.
//!
//! springrep has no sweep of a solid. A circle about the z axis meets the bar in one arc across the
//! plane y = 0, except beyond its outer face: there, past x = 11 and within √(11² + 0.5²) of the
//! axis, it meets the bar in two short arcs either side of y = 0, which the turn carries across the
//! gap between them. So the bar sweeps what the bar with that gap filled does, out to the cylinder
//! about the z axis through its outer edges; and that solid meets every circle about the z axis in
//! one arc across y = 0. The sweep is it where it starts and stops and its section there, the
//! rectangle 10 ≤ ρ ≤ √(11² + 0.5²), |z| ≤ 0.6, turned through the 170° (`shared::swept`, a turn).
use springrep::boolean::{difference, intersection};
use springrep::math::Frame;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, deg, rect, sector_block, swing, turned, unite, v3};

pub fn build() -> Result<Brep, String> {
    let reach = (11f64 * 11.0 + 0.25).sqrt();
    let reaching = boxed([10.0, -0.5, -0.6], [reach + 0.5, 0.5, 0.6], "bar")?;
    let around = cylinder(&Frame::WORLD.translated(v3(0.0, 0.0, -1.0)), reach, 2.0, "reach").map_err(e)?;
    let filled = intersection(&reaching, &around).map_err(e)?;
    let (from, to) = (deg(-140.0), deg(30.0));
    let swept = swing(&rect(10.0, -0.6, reach, 0.6), from, to - from, "notch")?;
    let notch = unite(&[&turned(&filled, from), &swept, &turned(&filled, to)])?;
    difference(&sector_block()?, &notch).map_err(e)
}

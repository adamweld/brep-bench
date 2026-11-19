//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a pin passes
//! through as it turns about the z axis from −140° to 140°: the cylinder of radius 0.5 about the x
//! axis from x = 6 to 11.
//!
//! springrep has no sweep of a solid. A circle about the z axis meets the pin in one arc across the
//! plane y = 0, except beyond its far end: there, past x = 11 and within √(11² + 0.5²) of the
//! origin, it meets the pin in two short arcs either side of y = 0, which the turn carries across
//! the gap between them. So the pin sweeps what the pin with that gap filled does, out to the
//! sphere about the origin through its far rim; and that solid meets every circle about the z axis
//! in one arc across y = 0. Its section there, the rectangle 6 ≤ ρ ≤ 11, |z| ≤ 0.5 capped by that
//! sphere, turned through the 280°, is the sweep within the block (`shared::swept`, a turn); the
//! pin is clear of the block where it starts and stops.
use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{arc, deg, line, outline, sector_block, swing};

pub fn build() -> Result<Brep, String> {
    let reach = (11f64 * 11.0 + 0.25).sqrt();
    let a = 0.5f64.atan2(11.0);
    let section = outline(vec![
        line((6.0, -0.5), (11.0, -0.5), "floor"),
        arc((0.0, 0.0), reach, -a, 2.0 * a, "end"),
        line((11.0, 0.5), (6.0, 0.5), "roof"),
        line((6.0, 0.5), (6.0, -0.5), "start"),
    ]);
    let (from, to) = (deg(-140.0), deg(140.0));
    let slot = swing(&section, from, to - from, "slot")?;
    difference(&sector_block()?, &slot).map_err(e)
}

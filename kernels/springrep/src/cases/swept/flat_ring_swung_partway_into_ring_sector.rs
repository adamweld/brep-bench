//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a ring passes
//! through as it turns about the z axis from −140° to 30°: the torus about the upright line through
//! (10, 0), its tube's centers on the circle of radius 1 in the plane z = 0, its tube of radius 0.4.
//!
//! springrep has no sweep of a solid. A circle about the z axis that crosses the ring's hole meets
//! the ring on both sides of it, at most a few degrees apart; the turn carries the arc behind the
//! hole across it, so the hole, filled, is swept too, and the ring sweeps what the ring with its
//! hole filled does. That solid of revolution meets every circle about the z axis in one arc,
//! across the plane through both axes, so the sweep is it where it starts and where it stops and
//! its section by that plane, a stadium from ρ = 8.6 to 11.4, turned through the 170°
//! (`shared::swept`, a turn).
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{
    arc, deg, line, outline, sector_block, swing, turned, unite, upright,
};

pub fn build() -> Result<Brep, String> {
    let r = 0.4;
    // the ring with its hole filled: the half stadium out from its axis, turned about it
    let meridian = outline(vec![
        line((0.0, -r), (1.0, -r), "bottom"),
        arc((1.0, 0.0), r, -PI / 2.0, PI, "tube"),
        line((1.0, r), (0.0, r), "top"),
        line((0.0, r), (0.0, -r), "axis"),
    ]);
    let filled = upright(&meridian, 10.0, 0.0, "ring")?;
    let section = outline(vec![
        line((9.0, -r), (11.0, -r), "floor"),
        arc((11.0, 0.0), r, -PI / 2.0, PI, "tube"),
        line((11.0, r), (9.0, r), "roof"),
        arc((9.0, 0.0), r, PI / 2.0, PI, "tube"),
    ]);
    let (from, to) = (deg(-140.0), deg(30.0));
    let swept = swing(&section, from, to - from, "notch")?;
    let notch = unite(&[&turned(&filled, from), &swept, &turned(&filled, to)])?;
    difference(&sector_block()?, &notch).map_err(e)
}

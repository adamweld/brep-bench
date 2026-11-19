//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a bull-nose
//! mill passes through as it turns about the z axis from −140° to 30°: a cylinder of radius 1 about
//! the upright line through (10, 0), from z = −0.5 to 0.5, its bottom corner rounded to 0.3.
//!
//! springrep has no sweep of a solid. The mill is a solid of revolution about an upright line, so
//! every circle about the z axis meets it in one arc, across the plane through both axes; the
//! sweep is the mill where it starts and where it stops and its section by that plane turned
//! through the 170° (`shared::swept`, a turn).
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{
    arc, deg, line, outline, sector_block, swing, turned, unite, upright,
};

pub fn build() -> Result<Brep, String> {
    let meridian = outline(vec![
        line((0.0, -0.5), (0.7, -0.5), "bottom"),
        arc((0.7, -0.2), 0.3, -PI / 2.0, PI / 2.0, "round"),
        line((1.0, -0.2), (1.0, 0.5), "side"),
        line((1.0, 0.5), (0.0, 0.5), "top"),
        line((0.0, 0.5), (0.0, -0.5), "axis"),
    ]);
    let mill = upright(&meridian, 10.0, 0.0, "mill")?;
    let section = outline(vec![
        line((9.3, -0.5), (10.7, -0.5), "bottom"),
        arc((10.7, -0.2), 0.3, -PI / 2.0, PI / 2.0, "round"),
        line((11.0, -0.2), (11.0, 0.5), "side"),
        line((11.0, 0.5), (9.0, 0.5), "top"),
        line((9.0, 0.5), (9.0, -0.2), "side"),
        arc((9.3, -0.2), 0.3, PI, PI / 2.0, "round"),
    ]);
    let (from, to) = (deg(-140.0), deg(30.0));
    let swept = swing(&section, from, to - from, "notch")?;
    let notch = unite(&[&turned(&mill, from), &swept, &turned(&mill, to)])?;
    difference(&sector_block()?, &notch).map_err(e)
}

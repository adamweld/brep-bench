//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a domed mill
//! passes through as it turns about the z axis from −140° to 30°: a cylinder of radius 1 about the
//! upright line through (10, 0) up to z = 3, ended below by a cap of the sphere of radius 2 about
//! (10, 0, 1.2).
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
    let rim = 1.2 - 3f64.sqrt();
    // the meridian: the cap's arc from its lowest point up to the sharp rim, the side, the top
    let meridian = outline(vec![
        arc((0.0, 1.2), 2.0, -PI / 2.0, PI / 6.0, "cap"),
        line((1.0, rim), (1.0, 3.0), "side"),
        line((1.0, 3.0), (0.0, 3.0), "top"),
        line((0.0, 3.0), (0.0, -0.8), "axis"),
    ]);
    let mill = upright(&meridian, 10.0, 0.0, "mill")?;
    let section = outline(vec![
        arc((10.0, 1.2), 2.0, -2.0 * PI / 3.0, PI / 3.0, "cap"),
        line((11.0, rim), (11.0, 3.0), "side"),
        line((11.0, 3.0), (9.0, 3.0), "top"),
        line((9.0, 3.0), (9.0, rim), "side"),
    ]);
    let (from, to) = (deg(-140.0), deg(30.0));
    let swept = swing(&section, from, to - from, "notch")?;
    let notch = unite(&[&turned(&mill, from), &swept, &turned(&mill, to)])?;
    difference(&sector_block()?, &notch).map_err(e)
}

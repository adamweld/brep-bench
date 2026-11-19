//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a drill
//! passes through as it turns about the z axis from −140° to 30°: the drill is a cylinder of radius
//! 1 about the upright line through (10, 0) up to z = 3, ended below in a 118° cone pointed at
//! (10, 0, −0.8).
//!
//! springrep has no sweep of a solid. The drill is a solid of revolution about an upright line, so
//! every circle about the z axis meets it in one arc, across the plane through both axes; the
//! sweep is the drill where it starts and where it stops and its section by that plane turned
//! through the 170° (`shared::swept`, a turn).
use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, polygon, sector_block, swing, turned, unite, upright};

pub fn build() -> Result<Brep, String> {
    let rim = -0.8 + 1.0 / deg(59.0).tan();
    // the drill's meridian, out from its axis
    let meridian = polygon(&[(0.0, -0.8), (1.0, rim), (1.0, 3.0), (0.0, 3.0)], "drill");
    let drill = upright(&meridian, 10.0, 0.0, "drill")?;
    // its section by the plane y = 0, both sides of its axis
    let section = polygon(
        &[(10.0, -0.8), (11.0, rim), (11.0, 3.0), (9.0, 3.0), (9.0, rim)],
        "notch",
    );
    let (from, to) = (deg(-140.0), deg(30.0));
    let swept = swing(&section, from, to - from, "notch")?;
    let notch = unite(&[&turned(&drill, from), &swept, &turned(&drill, to)])?;
    difference(&sector_block()?, &notch).map_err(e)
}

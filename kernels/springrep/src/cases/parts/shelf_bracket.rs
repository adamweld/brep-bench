//! An L-shaped bracket, a post 20 wide and 120 tall with an arm out to x = 100 along its top, the
//! inside corner rounded to 8, its outline in the xz plane extruded from y = −5 to 5.
use std::f64::consts::{FRAC_PI_2, PI};

use springrep::math::Vec2;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::parts::{line, xz};

pub fn build() -> Result<Brep, String> {
    let outline = Profile::outline(vec![
        line((0., 0.), (20., 0.), "foot"),
        line((20., 0.), (20., 92.), "post"),
        // the round about (28, 92), from 180° clockwise to 90°
        NamedSegment::arc(Vec2::new(28., 92.), 8., PI, -FRAC_PI_2, "round"),
        line((28., 100.), (100., 100.), "arm"),
        line((100., 100.), (100., 120.), "end"),
        line((100., 120.), (0., 120.), "top"),
        line((0., 120.), (0., 0.), "back"),
    ]);
    // the xz plane at y = 5 faces −y: extruded 10 along it, to y = −5
    extrude(&outline, &xz(5.)?, 10., "bracket").map_err(e)
}

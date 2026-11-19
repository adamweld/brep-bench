//! A stepped shaft turned about the z axis: radius 20 from z = 0 to 60, radius 12 on to 160, the
//! inside corner at the shoulder rounded to 4.
use std::f64::consts::{FRAC_PI_2, PI, TAU};

use springrep::math::{Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::parts::{line, xz};

pub fn build() -> Result<Brep, String> {
    let section = Profile::outline(vec![
        line((0., 0.), (20., 0.), "end"),
        line((20., 0.), (20., 60.), "thick"),
        line((20., 60.), (16., 60.), "shoulder"),
        // the round about (16, 64), from 270° clockwise to 180°
        NamedSegment::arc(Vec2::new(16., 64.), 4., 1.5 * PI, -FRAC_PI_2, "round"),
        line((12., 64.), (12., 160.), "thin"),
        line((12., 160.), (0., 160.), "end"),
        line((0., 160.), (0., 0.), "axis"),
    ]);
    revolve(&section, &xz(0.)?, Vec3::ZERO, Vec3::Z, TAU, "shaft").map_err(e)
}

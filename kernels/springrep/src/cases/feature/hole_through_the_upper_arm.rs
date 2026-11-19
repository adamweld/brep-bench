//! A C-shaped bracket, two arms 40 long joined by a back, with a hole of radius 4 drilled down
//! through its upper arm and no further.
//!
//! springrep has no hole that runs to the next face it comes out of, so the hole is cut as a
//! cylinder from the upper arm's top down into the gap, where it comes out: through the upper arm
//! and no further.
use springrep::boolean::difference;
use springrep::math::Vec2;
use springrep::primitive::cylinder;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    // the section in (x, z), drawn on the plane y = 10 facing −y so the frame's y is +z, and
    // extruded 20 along −y to y = −10
    let xz = frame([0., 10., 0.], [0., -1., 0.], [1., 0., 0.])?;
    let p = |x: f64, z: f64| Vec2::new(x, z);
    let section = Profile::polygon(
        &[
            p(0., 0.),
            p(40., 0.),
            p(40., 6.),
            p(6., 6.),
            p(6., 24.),
            p(40., 24.),
            p(40., 30.),
            p(0., 30.),
        ],
        &["bottom", "lower_end", "lower_top", "back_face", "upper_under", "upper_end", "top", "back"],
    );
    let bracket = extrude(&section, &xz, 20., "bracket").map_err(e)?;
    // from the middle of the gap up through the top, on the axis through (28, 0)
    let drill = cylinder(&at([28., 0., 15.]), 4., 20., "hole").map_err(e)?;
    difference(&bracket, &drill).map_err(e)
}

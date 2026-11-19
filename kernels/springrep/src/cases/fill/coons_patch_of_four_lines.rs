//! The solid over the square from (0, 0) to (10, 10) under the Coons patch of the four straight
//! edges from (0, 0, 5) to (10, 0, 5), (10, 10, 15), (0, 10, 5) and back: the saddle
//! z = 5 + xy/10, with upright walls and a flat floor.
//!
//! springrep has no fill of a loop of curves, but with straight edges the bilinear Coons patch is
//! the ruled surface between the edges on y = 0 and y = 10, as the case allows: a ruled loft
//! between the walls y = 0 and y = 10, whose top is that ruled surface and whose other sides are
//! the flat walls x = 0 and x = 10 and the floor.
use springrep::loft::{Section, loft};
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::topo::Brep;

use crate::shared::{e, frame};

/// The wall on the plane y = `y`, drawn in (x, z), from the floor up to heights `h0` at x = 0
/// and `h1` at x = 10.
fn wall(y: f64, h0: f64, h1: f64) -> Result<Section, String> {
    // facing −y, so the frame's y is +z
    let plane = frame([0., y, 0.], [0., -1., 0.], [1., 0., 0.])?;
    let p = |x: f64, z: f64| Vec2::new(x, z);
    let outline = Profile::polygon(
        &[p(0., 0.), p(10., 0.), p(10., h1), p(0., h0)],
        &["floor", "x10", "top", "x0"],
    );
    Ok(Section { profile: outline, frame: plane })
}

pub fn build() -> Result<Brep, String> {
    let sections = [wall(0., 5., 5.)?, wall(10., 5., 15.)?];
    loft(&sections, true, "fill").map_err(e)
}

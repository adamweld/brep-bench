//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a drum
//! passes through as it is screwed about the z axis, rising 0.5 a turn, from a turn of −140° to one
//! of 140°: the cylinder of radius 1 about the upright line through (10, 0), from z = −0.5 to 0.5.
//!
//! springrep has no sweep of a solid; the drum is an upright prism, so its sweep under a screw about
//! the upright axis is the drum where it starts and stops, its flat ends' twisted extrusions and
//! its section in the plane through both axes, the rectangle 9 ≤ ρ ≤ 11, |z| ≤ 0.5, screwed: every
//! circle about the z axis meets the drum in one arc across that plane
//! (`shared::swept::screwed_prism`).
use springrep::boolean::difference;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, screwed_prism, sector_block};

pub fn build() -> Result<Brep, String> {
    let disc = Profile::circle(Vec2::new(10.0, 0.0), 2.0);
    let groove = screwed_prism(&disc, -0.5, 0.5, 0.5, deg(-140.0), deg(140.0), &[(0.0, 9.0, 11.0)], true)
        .map_err(|why| format!("springrep's union of the sweep's pieces fails: {why}"))?;
    difference(&sector_block()?, &groove).map_err(e)
}

//! The rod of radius 5 about the z axis from z = 0 to z = 20, less everything the bar
//! 4.5 ≤ x ≤ 12, |y| ≤ 8, |z| ≤ 0.45 passes through as it is screwed about the z axis, rising 1 a
//! turn, from a turn of −180° to one of 7380°: a thread whose crests are sharp.
//!
//! springrep has no sweep of a solid. The bar is 0.9 thick along the screw's axis, which the screw
//! rises in a turn of 324°: at each height of the rod the bar passes wholly, staying there through
//! 324°, and what it covers there is its level section turned through 324°, turned on with the
//! height — a twisted extrusion (`shared::swept::screwed_bar`). Turned that far the section closes
//! round the axis beyond 4.5/cos 18°, where its tip's lower half where it starts meets its upper
//! half where it stops: what it covers is everything out to the rod and beyond, but for the disc
//! of radius 4.5 and the notch between those two halves.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{arc, deg, line, outline, rod, screwed_bar};

pub fn build() -> Result<Brep, String> {
    let delta = 2.0 * PI * 0.9;
    let (s, c) = delta.sin_cos();
    let tip = 4.5;
    // where the turned tip's upper half crosses the tip's lower half where it starts
    let crest = (tip, -tip * deg(18.0).tan());
    let left_open = outline(vec![
        arc((0.0, 0.0), tip, 0.0, delta, "root"),
        line((tip * c, tip * s), crest, "leading"),
        line(crest, (tip, 0.0), "trailing"),
    ]);
    let section = Profile::circle(Vec2::ZERO, 11.0).with_holes(left_open.loops);
    let groove = screwed_bar(&section, 0.9, 1.0, deg(-180.0), deg(7380.0), delta)?;
    difference(&rod()?, &groove).map_err(e)
}

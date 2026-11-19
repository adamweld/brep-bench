//! The rod of radius 5 about the z axis from z = 0 to z = 20 less everything the box
//! 2.5 ≤ x ≤ 3.5, |y| ≤ 0.5, 2 ≤ z ≤ 3 passes through as it is screwed about the z axis, rising 4
//! a turn, from a turn of 0° to one of 720°: a sealed helical tunnel.
//!
//! springrep has no sweep of a solid; the box is an upright prism, so its sweep under a screw about
//! the upright axis is the box where it starts and stops, its level faces' twisted extrusions and
//! its sections in planes through the axis screwed (`shared::swept::screwed_prism`). A circle about
//! the axis meets the box's square in one arc across the x axis, out to 3.5; beyond, out to its
//! outer corners, in two short arcs, each crossing the ray from the axis through its corner. So
//! the sections are the box's along the x axis and along the rays through its two outer corners.
use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, rect, rod, screwed_prism};

pub fn build() -> Result<Brep, String> {
    let square = rect(2.5, -0.5, 3.5, 0.5);
    let a = 0.5f64.atan2(3.5);
    let (near, far) = (2.5 / a.cos(), (3.5f64 * 3.5 + 0.25).sqrt());
    let sections = [(0.0, 2.5, 3.5), (a, near, far), (-a, near, far)];
    let tunnel = screwed_prism(&square, 2.0, 3.0, 4.0, 0.0, deg(720.0), &sections, true)
        .map_err(|why| format!("springrep's union of the sweep's pieces fails: {why}"))?;
    difference(&rod()?, &tunnel).map_err(e)
}

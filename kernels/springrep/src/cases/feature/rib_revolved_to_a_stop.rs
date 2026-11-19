//! A hub of radius 10 with a stop (the wedge within 25 of the axis between 120° and 135°), and a
//! rib, the rectangle 5 ≤ x ≤ 20, 0 ≤ z ≤ 4 on y = 0, turned about the z axis until it meets the
//! stop, all joined.
//!
//! springrep has no revolution up to a face, so the rib is turned through the 120° at which the
//! stop's face lies, and the union joins it flat against that face.
use std::f64::consts::PI;

use springrep::boolean::union_all;
use springrep::math::{Vec2, Vec3};
use springrep::primitive::cylinder;
use springrep::profile::Profile;
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

/// The rectangle from (r0, 0) to (r1, h) on the half-plane at angle `from` about the z axis,
/// turned through `angle` toward +y.
fn turned(r0: f64, r1: f64, h: f64, from: f64, angle: f64, feature: &str) -> Result<Brep, String> {
    // the sketch plane holds the axis: its x is the half-plane's direction, its y the z axis
    let x = [from.cos(), from.sin(), 0.];
    let plane = frame([0., 0., 0.], [from.sin(), -from.cos(), 0.], x)?;
    let rect = Profile::rect(Vec2::new((r0 + r1) / 2., h / 2.), r1 - r0, h);
    revolve(&rect, &plane, Vec3::ZERO, Vec3::Z, angle, feature).map_err(e)
}

pub fn build() -> Result<Brep, String> {
    let hub = cylinder(&at([0., 0., 0.]), 10., 10., "hub").map_err(e)?;
    let deg = PI / 180.;
    let stop = turned(0., 25., 10., 120. * deg, 15. * deg, "stop")?;
    let rib = turned(5., 20., 4., 0., 120. * deg, "rib")?;
    union_all(&[&hub, &stop, &rib]).map_err(e)
}

//! The solid a ball of radius 2 centered at (1, 0, 0) fills as it turns 120° about the z axis,
//! which passes through it.
//!
//! springrep has no sweep of a solid. A circle about the z axis meets the ball in one arc about
//! its point in the half-plane y = 0, x ≥ 0 (its nearest to the ball's center), or lies wholly in
//! the ball; so the sweep is the ball where it starts and where it stops and its section by that
//! half-plane, the part of the disc of radius 2 about (1, 0) on the axis's side x ≥ 0, turned
//! through 120° (`shared::swept`, a turn).
use std::f64::consts::PI;

use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::swept::{arc, deg, line, outline, swing, turned, unite};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&at([1.0, 0.0, 0.0]), 2.0, "ball").map_err(e)?;
    let h = 3f64.sqrt();
    // the disc's rim from where it crosses the axis below, round through (3, 0), to above, and the
    // axis back down
    let section = outline(vec![
        arc((1.0, 0.0), 2.0, -2.0 * PI / 3.0, 4.0 * PI / 3.0, "rim"),
        line((0.0, h), (0.0, -h), "axis"),
    ]);
    let swept = swing(&section, 0.0, deg(120.0), "swept")?;
    unite(&[&ball, &swept, &turned(&ball, deg(120.0))])
}

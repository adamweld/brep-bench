//! The solid a square bar fills as it turns a quarter turn about the z axis: the prism from z = 0
//! to 3 on the square with corners (4, 0), (6, 2), (8, 0) and (6, −2), stood on a corner.
//!
//! springrep has no sweep of a solid. Every circle about the z axis meets the bar in one arc,
//! across its diagonal on the x axis, so the sweep is the bar where it starts and where it stops
//! and its section by the plane y = 0, the rectangle 4 ≤ ρ ≤ 8, 0 ≤ z ≤ 3, turned through the
//! quarter turn (`shared::swept`, a turn).
use std::f64::consts::FRAC_PI_2;

use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::swept::{polygon, rect, swing, turned, unite};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let square = polygon(&[(4.0, 0.0), (6.0, -2.0), (8.0, 0.0), (6.0, 2.0)], "side");
    let bar = extrude(&square, &at([0.0, 0.0, 0.0]), 3.0, "bar").map_err(e)?;
    let swept = swing(&rect(4.0, 0.0, 8.0, 3.0), 0.0, FRAC_PI_2, "swept")?;
    unite(&[&bar, &swept, &turned(&bar, FRAC_PI_2)])
}

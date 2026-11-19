//! The block (0, −5, 0)–(20, 5, 10) less everything an end mill passes through as it slides along
//! the x axis by 30: the cylinder of radius 2 about the upright line through (−5, 0), from z = 7
//! to 15.
//!
//! springrep has no sweep of a solid. Every line along x through the mill crosses its section by
//! the plane x = −5 through its axis, so the mill sweeps itself where it starts and stops and that
//! section, the rectangle |y| ≤ 2, 7 ≤ z ≤ 15, carried along 30: a box (`shared::swept`, a slide).
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::swept::{boxed, shifted, unite, v3};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let block = boxed([0.0, -5.0, 0.0], [20.0, 5.0, 10.0], "block")?;
    let mill = cylinder(&at([-5.0, 0.0, 7.0]), 2.0, 8.0, "mill").map_err(e)?;
    let middle = boxed([-5.0, -2.0, 7.0], [25.0, 2.0, 15.0], "slot")?;
    let slot = unite(&[&mill, &middle, &shifted(&mill, v3(30.0, 0.0, 0.0))])?;
    difference(&block, &slot).map_err(e)
}

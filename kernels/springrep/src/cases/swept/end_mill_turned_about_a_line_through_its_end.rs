//! The box (−10, −10, −5)–(10, 10, 0) less everything an end mill passes through as it turns 120°
//! about the z axis: the cylinder of radius 3 about the upright line through (1.5, 0), from z = −2
//! to 4, which the z axis passes through.
//!
//! springrep has no sweep of a solid. A circle about the z axis meets the mill in one arc about its
//! point in the half-plane y = 0, x ≥ 0 (its nearest to the mill's axis), or lies wholly in it; so
//! the sweep is the mill where it starts and where it stops and its section by that half-plane,
//! the rectangle 0 ≤ ρ ≤ 4.5, −2 ≤ z ≤ 4, turned through 120° (`shared::swept`, a turn).
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::swept::{boxed, deg, rect, swing, turned, unite};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let block = boxed([-10.0, -10.0, -5.0], [10.0, 10.0, 0.0], "block")?;
    let mill = cylinder(&at([1.5, 0.0, -2.0]), 3.0, 6.0, "mill").map_err(e)?;
    let swept = swing(&rect(0.0, -2.0, 4.5, 4.0), 0.0, deg(120.0), "pocket")?;
    let pocket = unite(&[&mill, &swept, &turned(&mill, deg(120.0))])?;
    difference(&block, &pocket).map_err(e)
}

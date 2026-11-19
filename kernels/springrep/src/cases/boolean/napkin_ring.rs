//! A napkin ring: the ball of radius 5 about the origin bored through along z by a hole of radius 3.
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::boolean::ball;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let whole = ball([0., 0., 0.], 5., "ball")?;
    // the drill runs past the ball at both ends
    let drill = cylinder(&at([0., 0., -6.]), 3., 12., "bore").map_err(e)?;
    difference(&whole, &drill).map_err(e)
}

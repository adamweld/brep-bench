//! A napkin ring, the ball of radius 5 about the origin bored through along z by a hole of radius
//! 3, grown by 1: the sphere grows to radius 6, the bore narrows to 2, and each rim becomes a piece
//! of a torus.
use springrep::boolean::difference;
use springrep::primitive::{cylinder, sphere};
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)?;
    let bore = cylinder(&at([0., 0., -6.]), 3., 12., "bore").map_err(e)?;
    let ring = difference(&ball, &bore).map_err(e)?;
    grown(&ring, 1.)
}

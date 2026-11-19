//! A ball of radius 5 bored through along the z axis by a hole of radius 3, the ball's poles at
//! (3, 0, 4) and (−3, 0, −4), on the ring's rims.
use springrep::boolean::difference;
use springrep::primitive::{cylinder, sphere};
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    let poles = frame([0., 0., 0.], [3., 0., 4.], [0., 1., 0.])?;
    let ball = sphere(&poles, 5., "ball").map_err(e)?;
    let bore = cylinder(&at([0., 0., -6.]), 3., 12., "bore").map_err(e)?;
    difference(&ball, &bore).map_err(e)
}

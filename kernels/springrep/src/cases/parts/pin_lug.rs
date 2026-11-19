//! A lug, the plate (0, −5, 0)–(80, 5, 40), with a pin hole of radius 8 along y about the line
//! x = 60, z = 20.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    let plate = block(&at([0., -5., 0.]), Vec3::new(80., 10., 40.), "lug").map_err(e)?;
    let pin = cylinder(&frame([60., -6., 20.], [0., 1., 0.], [1., 0., 0.])?, 8., 12., "pin")
        .map_err(e)?;
    difference(&plate, &pin).map_err(e)
}

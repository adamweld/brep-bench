//! The part common to the cube [0, 10]³ and its inscribed ball, of radius 5 about (5, 5, 5): the
//! whole ball.
use springrep::boolean::intersection;
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::tangent::between;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let cube = between([0., 0., 0.], [10., 10., 10.], "cube")?;
    let ball = sphere(&at([5., 5., 5.]), 5., "ball").map_err(e)?;
    intersection(&cube, &ball).map_err(e)
}

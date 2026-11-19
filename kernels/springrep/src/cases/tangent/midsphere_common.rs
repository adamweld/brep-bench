//! The part common to the cube [-5, 5]³ and its midsphere, the ball of radius 5√2 about the origin
//! that touches each edge at its midpoint.
use springrep::boolean::intersection;
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::tangent::between;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let cube = between([-5., -5., -5.], [5., 5., 5.], "cube")?;
    let ball = sphere(&at([0., 0., 0.]), 5. * 2f64.sqrt(), "ball").map_err(e)?;
    intersection(&cube, &ball).map_err(e)
}

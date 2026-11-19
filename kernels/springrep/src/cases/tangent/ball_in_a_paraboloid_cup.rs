//! The solid paraboloid x² + y² ≤ 10z, z ≤ 5, united with the ball of radius 5 about (0, 0, 5),
//! whose curvature matches the cup's at its vertex: they touch there only, to more than first order.
use springrep::boolean::union;
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::tangent::paraboloid;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // x² + y² = 10z: a focal length of 2.5
    let cup = paraboloid(2.5, 5., "cup")?;
    // the ball's poles on the z axis: the lower one is the point of contact
    let ball = sphere(&at([0., 0., 5.]), 5., "ball").map_err(e)?;
    union(&cup, &ball).map_err(e)
}

//! The half of the ball of radius 5 about the origin where z ≥ 0 united with the half where x ≥ 0:
//! their curved faces coincide over a quarter of the sphere.
use springrep::boolean::{intersection, union};
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::tangent::half_space;
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the first ball's poles on the z axis, the second's on the x axis
    let poles_z = sphere(&frame([0., 0., 0.], [0., 0., 1.], [1., 0., 0.])?, 5., "upper").map_err(e)?;
    let poles_x = sphere(&frame([0., 0., 0.], [1., 0., 0.], [0., 1., 0.])?, 5., "east").map_err(e)?;
    let upper = intersection(&poles_z, &half_space([0., 0., 0.], [0., 0., 1.], 10., "above")?)
        .map_err(e)?;
    let east = intersection(&poles_x, &half_space([0., 0., 0.], [1., 0., 0.], 10., "beyond")?)
        .map_err(e)?;
    union(&upper, &east).map_err(e)
}

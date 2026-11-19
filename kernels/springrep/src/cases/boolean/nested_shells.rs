//! Three bodies nested in each other's hollows: the ball of radius 10 less one of radius 8, the
//! ball of radius 6 less one of radius 4, and a ball of radius 2, no two sharing a centre.
use springrep::boolean::{difference, union};
use springrep::topo::Brep;

use crate::shared::boolean::ball;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let outer = difference(
        &ball([0., 0., 0.], 10., "b10")?,
        &ball([1., 0., 0.], 8., "b8")?,
    )
    .map_err(e)?;
    let middle = difference(
        &ball([1., 1., 0.], 6., "b6")?,
        &ball([1., 1., 1.], 4., "b4")?,
    )
    .map_err(e)?;
    let inner = ball([1., 1.5, 1.5], 2., "b2")?;
    let two = union(&outer, &middle).map_err(e)?;
    union(&two, &inner).map_err(e)
}

//! The half of the torus about z (middle radius 10, tube radius 3) where z ≥ 0 united with the half
//! where x ≥ 0: their curved faces coincide over a quarter of the torus.
use springrep::boolean::{intersection, union};
use springrep::primitive::torus;
use springrep::topo::Brep;

use crate::shared::tangent::half_space;
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the second torus turned a quarter turn about z, so the seams differ
    let first = torus(&frame([0., 0., 0.], [0., 0., 1.], [1., 0., 0.])?, 10., 3., "first").map_err(e)?;
    let second = torus(&frame([0., 0., 0.], [0., 0., 1.], [0., 1., 0.])?, 10., 3., "second").map_err(e)?;
    let upper = intersection(&first, &half_space([0., 0., 0.], [0., 0., 1.], 20., "above")?)
        .map_err(e)?;
    let east = intersection(&second, &half_space([0., 0., 0.], [1., 0., 0.], 20., "beyond")?)
        .map_err(e)?;
    union(&upper, &east).map_err(e)
}

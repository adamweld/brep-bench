//! Two separate tori of middle radius 3 and tube radius 0.5, linked: one about the z axis, one
//! about the line through (3, 0, 0) along y. Their union keeps both bodies.
use springrep::boolean::union;
use springrep::primitive::torus;
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    let first = torus(&at([0., 0., 0.]), 3., 0.5, "first").map_err(e)?;
    let second = torus(&frame([3., 0., 0.], [0., 1., 0.], [1., 0., 0.])?, 3., 0.5, "second").map_err(e)?;
    union(&first, &second).map_err(e)
}

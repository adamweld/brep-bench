//! The torus about the z axis of middle radius 4 and tube radius 2, grown by 3: the grown tube, of
//! radius 5, closes over the hole, leaving an apple with a dimple above and below.
//! springrep 1.0.1 builds the grown torus's face off its own edges (1.1 mm) where it crosses the
//! axis, which its checker refuses.
use springrep::primitive::torus;
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ring = torus(&at([0., 0., 0.]), 4., 2., "ring").map_err(e)?;
    grown(&ring, 3.)
}

//! The torus about the z axis of middle radius 10 and tube radius 2, less everything where x < 0.
use springrep::boolean::difference;
use springrep::primitive::torus;
use springrep::topo::Brep;

use crate::shared::boolean::between;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ring = torus(&at([0., 0., 0.]), 10., 2., "torus").map_err(e)?;
    // the half-space x < 0, as a block past the torus on every other side
    let west = between([-20., -20., -20.], [0., 20., 20.], "west")?;
    difference(&ring, &west).map_err(e)
}

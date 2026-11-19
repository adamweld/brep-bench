//! The ball of radius 5 about the origin with everything below the xy plane cut away.
use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::boolean::{ball, between};
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let whole = ball([0., 0., 0.], 5., "ball")?;
    // the half-space z < 0, as a block past the ball on every other side
    let below = between([-10., -10., -10.], [10., 10., 0.], "below")?;
    difference(&whole, &below).map_err(e)
}

//! The ball of radius 20 about the origin bored through parallel to y by a hole of radius 6 whose
//! axis is the line x = 5, z = 0.
use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::boolean::{ball, rod};
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let whole = ball([0., 0., 0.], 20., "ball")?;
    // the drill runs past the ball at both ends
    let drill = rod([5., -21., 0.], [0., 1., 0.], 6., 42., "bore")?;
    difference(&whole, &drill).map_err(e)
}

//! A ball of radius 5 about the origin, its poles on the z axis, less the cap beyond the plane
//! x + z = 5, whose rim runs through the pole (0, 0, 5).
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::singular::cut_away;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)?;
    cut_away(&ball, [5., 0., 0.], [1., 0., 1.])
}

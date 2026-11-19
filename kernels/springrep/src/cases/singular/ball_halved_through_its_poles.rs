//! A ball of radius 5 about the origin, its poles on the z axis, less everything with x < 0: the
//! cut runs along a meridian from pole to pole.
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::singular::cut_away;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)?;
    cut_away(&ball, [0., 0., 0.], [-1., 0., 0.])
}

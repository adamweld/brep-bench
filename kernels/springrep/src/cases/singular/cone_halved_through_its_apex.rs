//! A cone on the z axis, base radius 5 at z = 0 and apex at (0, 0, 10), less everything with
//! x < 0: the cut holds the axis and the apex.
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::singular::cut_away;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let solid = cone(&at([0., 0., 0.]), 5., 0., 10., "cone").map_err(e)?;
    cut_away(&solid, [0., 0., 0.], [-1., 0., 0.])
}

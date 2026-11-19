//! A cone on the z axis, base radius 5 at z = 0 and apex at (0, 0, 10), less everything beyond
//! the plane 4x + z = 10 through its apex.
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::singular::cut_away;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let solid = cone(&at([0., 0., 0.]), 5., 0., 10., "cone").map_err(e)?;
    cut_away(&solid, [0., 0., 10.], [4., 0., 1.])
}

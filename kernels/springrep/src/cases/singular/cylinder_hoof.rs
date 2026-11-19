//! A hoof: the cylinder of radius 5 on the z axis from z = −1 to 10, kept above z = 0 and below
//! the plane z = x·tan 30° through the y axis.
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::singular::cut_away;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let drum = cylinder(&at([0., 0., -1.]), 5., 11., "drum").map_err(e)?;
    let above = cut_away(&drum, [0., 0., 0.], [0., 0., -1.])?;
    let tan = 30f64.to_radians().tan();
    cut_away(&above, [0., 0., 0.], [-tan, 0., 1.])
}

//! A pipe of radius 1 along three turns of the helix of radius 10 about the z axis, rising 5 a
//! turn, each end closed by a half ball: everything within 1 of the helix.
use springrep::boolean::union_all;
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::sweep::helical_pipe;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let pipe = helical_pipe()?;
    let start = sphere(&at([10., 0., 0.]), 1., "start").map_err(e)?;
    let end = sphere(&at([10., 0., 15.]), 1., "end").map_err(e)?;
    union_all(&[&pipe, &start, &end]).map_err(e)
}

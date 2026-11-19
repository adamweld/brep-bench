//! A cylinder of radius 5 from z = 0 to 10, the rim of its top rounded to radius 5, its own
//! radius: the top shrinks to a point and the round is a hemisphere about (0, 0, 5).
use springrep::blend::fillet;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let body = cylinder(&at([0., 0., 0.]), 5., 10., "cyl").map_err(e)?;
    fillet(&body, &[("cyl.end", "cyl.side")], 5., "round").map_err(e)
}

//! A cylinder of radius 5 from z = 0 to 10, the rim of its top chamfered 5 on both faces, its own
//! radius: the top shrinks to its center and the chamfer is a cone with its apex at (0, 0, 10).
use springrep::blend::chamfer;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let body = cylinder(&at([0., 0., 0.]), 5., 10., "cyl").map_err(e)?;
    chamfer(&body, &[("cyl.end", "cyl.side")], 5., "chamfer").map_err(e)
}

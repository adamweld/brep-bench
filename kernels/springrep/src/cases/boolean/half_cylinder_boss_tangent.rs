//! The cube [0, 10]³ united with the cylinder of radius 5 and height 10 about x = 10, y = 5: a cube
//! with a half-round end, the cylinder touching the faces y = 0 and y = 10 along the cube's edges.
use springrep::boolean::union;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::boolean::between;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let cube = between([0., 0., 0.], [10., 10., 10.], "cube")?;
    let boss = cylinder(&at([10., 5., 0.]), 5., 10., "boss").map_err(e)?;
    union(&cube, &boss).map_err(e)
}

//! Five faces of the cube from the origin to (10, 10, 10), an open tray, a sheet, thickened by 1 into
//! the cube: the cube less the box from (1, 1, 1) to (9, 9, 10).
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::thicken::thickened;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the tray is five faces of the cube, all but its top (`end`)
    let cube = block(&at([0., 0., 0.]), Vec3::new(10., 10., 10.), "cube").map_err(e)?;
    let tray = ["cube.start", "cube.u_min", "cube.u_max", "cube.v_min", "cube.v_max"];
    // a negative thickness thickens into the cube
    thickened(&cube, &tray, -1.)
}

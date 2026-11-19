//! Five faces of the cube from the origin to (10, 10, 10), an open tray, a sheet, thickened by 1 out
//! of the cube with its edges rounded: every point outside the cube within 1 of the tray, up to the
//! plane of its open top.
//!
//! springrep's thicken extends the moved faces until they meet, so its edges are sharp. The rounded
//! slab is the cube's offset by 1 (its faces moved out, each convex edge rounded to a quarter
//! cylinder and each corner to an eighth of a ball), less the half-space above z = 10, less the cube.
use springrep::boolean::{difference, half_space};
use springrep::edit::offset;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let cube = block(&at([0., 0., 0.]), Vec3::new(10., 10., 10.), "cube").map_err(e)?;
    let grown = offset(&cube, 1., "grown").map_err(e)?;
    let above = half_space(&grown, Vec3::new(0., 0., 10.), Vec3::Z).map_err(e)?;
    let cup = difference(&grown, &above).map_err(e)?;
    difference(&cup, &cube).map_err(e)
}

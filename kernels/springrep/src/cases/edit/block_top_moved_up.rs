//! The box from the origin to (40, 30, 10) drilled through by a hole of radius 5, its top face then
//! moved up 6: the sides and the hole's wall run on up to meet it.
use springrep::boolean::difference;
use springrep::edit::move_faces_by;
use springrep::math::{Transform, Vec3};
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::edit::faces;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(40., 30., 10.), "block").map_err(e)?;
    let drill = cylinder(&at([20., 15., -1.]), 5., 12., "hole").map_err(e)?;
    let drilled = difference(&b, &drill).map_err(e)?;
    let top = faces(&drilled, &["block.end"])?;
    move_faces_by(&drilled, &top, &Transform::translation(Vec3::new(0., 0., 6.))).map_err(e)
}

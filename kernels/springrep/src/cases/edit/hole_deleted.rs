//! The box from the origin to (20, 10, 10) drilled through by a hole of radius 3; the hole's wall
//! then deleted, the top and the bottom closing over it.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::edit::delete_named;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(20., 10., 10.), "block").map_err(e)?;
    let drill = cylinder(&at([10., 5., -1.]), 3., 12., "hole").map_err(e)?;
    let drilled = difference(&b, &drill).map_err(e)?;
    delete_named(&drilled, &["hole.side"])
}

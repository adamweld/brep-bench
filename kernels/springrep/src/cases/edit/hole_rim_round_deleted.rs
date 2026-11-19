//! The box from the origin to (20, 10, 10) drilled through by a hole of radius 2, its top rim
//! rounded to radius 1; the round then deleted, the rim sharp again.
use springrep::blend::fillet;
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(20., 10., 10.), "block").map_err(e)?;
    let drill = cylinder(&at([10., 5., -1.]), 2., 12., "hole").map_err(e)?;
    let drilled = difference(&b, &drill).map_err(e)?;
    let rounded = fillet(&drilled, &[("block.end", "hole.side")], 1., "round").map_err(e)?;
    delete_feature(&rounded, "round").map_err(|why| {
        format!("{why} (the hole's wall, hollow: springrep closes a gap only against planes and cylinders bulging outward)")
    })
}

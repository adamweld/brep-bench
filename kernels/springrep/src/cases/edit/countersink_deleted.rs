//! The box from the origin to (20, 10, 10) drilled through by a hole of radius 2, countersunk at
//! the top to radius 4; the countersink's cone then deleted, the wall running up to the top again.
use springrep::boolean::difference_all;
use springrep::math::Vec3;
use springrep::primitive::{block, cone, cylinder};
use springrep::topo::Brep;

use crate::shared::edit::delete_named;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(20., 10., 10.), "block").map_err(e)?;
    let drill = cylinder(&at([10., 5., -1.]), 2., 12., "hole").map_err(e)?;
    // the countersink's cone, from radius 2 at z = 8 widening 1 a unit up, on past the top
    let sink = cone(&at([10., 5., 8.]), 2., 5., 3., "sink").map_err(e)?;
    let part = difference_all(&b, &[&drill, &sink]).map_err(e)?;
    delete_named(&part, &["sink.side"]).map_err(|why| {
        format!("{why} (the hole's wall, hollow: springrep closes a gap only against planes and cylinders bulging outward)")
    })
}

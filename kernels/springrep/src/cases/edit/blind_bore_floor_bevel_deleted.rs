//! A block with a blind bore of radius 4 down to z = 4, the edge round its floor beveled by a cone
//! from z = 5 down to radius 3; the bevel then deleted, the floor and the wall meeting again.
use springrep::boolean::{difference, union};
use springrep::math::Vec3;
use springrep::primitive::{block, cone, cylinder};
use springrep::topo::Brep;

use crate::shared::edit::delete_named;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([-10., -10., 0.]), Vec3::new(20., 20., 10.), "block").map_err(e)?;
    // the cutter: the bore from z = 5 up through the top, on the frustum that makes its bevel
    let bore = cylinder(&at([0., 0., 5.]), 4., 6., "bore").map_err(e)?;
    let bevel = cone(&at([0., 0., 4.]), 3., 4., 1., "bevel").map_err(e)?;
    let cutter = union(&bore, &bevel).map_err(e)?;
    let part = difference(&b, &cutter).map_err(e)?;
    delete_named(&part, &["bevel.side"]).map_err(|why| {
        format!("{why} (the bore's wall, hollow: springrep closes a gap only against planes and cylinders bulging outward)")
    })
}

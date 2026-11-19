//! A block with a blind bore of radius 4 down to z = 4, the edge round its floor rounded to radius
//! 1; the round then deleted, the floor and the wall meeting again in a sharp edge.
use springrep::blend::fillet;
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([-10., -10., 0.]), Vec3::new(20., 20., 10.), "block").map_err(e)?;
    let bore = cylinder(&at([0., 0., 4.]), 4., 7., "bore").map_err(e)?;
    let part = difference(&b, &bore).map_err(e)?;
    // the bore's floor is the cutter's own floor, `bore.start`
    let rounded = fillet(&part, &[("bore.start", "bore.side")], 1., "round").map_err(e)?;
    delete_feature(&rounded, "round").map_err(|why| {
        format!("{why} (the bore's wall, hollow: springrep closes a gap only against planes and cylinders bulging outward)")
    })
}

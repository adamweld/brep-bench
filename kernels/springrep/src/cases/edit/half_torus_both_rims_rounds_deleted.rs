//! The half above z = 0 of the torus whose tube of radius 2 runs round the circle of radius 6, both
//! rims of its flat rounded to radius 1/2; both rounds then deleted, giving the half torus back.
use springrep::blend::fillet;
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, torus};
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ring = torus(&at([0., 0., 0.]), 6., 2., "ring").map_err(e)?;
    let below = block(&at([-10., -10., -3.]), Vec3::new(20., 20., 3.), "cut").map_err(e)?;
    let half = difference(&ring, &below).map_err(e)?;
    // the flat is the cutter's top; the pair takes both its rims, the inner and the outer
    let rounded = fillet(&half, &[("cut.end", "ring.tube")], 0.5, "round").map_err(e)?;
    delete_feature(&rounded, "round")
}

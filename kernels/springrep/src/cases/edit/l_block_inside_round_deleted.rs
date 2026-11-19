//! An L-shaped block 8 tall, its inside corner rounded to radius 2; the round then deleted, the
//! corner sharp again.
//!
//! Refused: springrep's `edit::delete_faces` fills gaps, it does not take material away.
use springrep::blend::fillet;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let outline: Vec<Vec2> = [(0., 0.), (20., 0.), (20., 5.), (5., 5.), (5., 15.), (0., 15.)]
        .iter()
        .map(|&(x, y)| Vec2::new(x, y))
        .collect();
    let sides = ["front", "end", "tread", "riser", "back", "side"];
    let l = extrude(&Profile::polygon(&outline, &sides), &at([0., 0., 0.]), 8., "l").map_err(e)?;
    let _rounded = fillet(&l, &[("l.tread", "l.riser")], 2., "round").map_err(e)?;
    // refused: springrep deletes faces only where the gap they leave is filled
    Err("the round in the L's inside corner is concave, material the fillet added: deleting it must take \
         that material away, and springrep's delete_faces only closes a gap by joining material to \
         the solid (its faces must be a recess, a hole or a corner cut away), so it would leave the \
         round in place"
        .into())
}

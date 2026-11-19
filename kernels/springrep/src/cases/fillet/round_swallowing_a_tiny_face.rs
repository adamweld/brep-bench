//! A block 10 by 10 by 5 whose vertical edge at x = 10, y = 10 is broken by a flat at 45° 0.0001
//! back along each side, the flat's two edges rounded together to radius 1: the round runs from
//! side to side and the flat is gone.
use springrep::blend::fillet;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let outline = [(0., 0.), (10., 0.), (10., 9.9999), (9.9999, 10.), (0., 10.)]
        .map(|(x, y)| Vec2::new(x, y));
    let section = Profile::polygon(&outline, &["south", "east", "flat", "north", "west"]);
    let block = extrude(&section, &at([0., 0., 0.]), 5., "block").map_err(e)?;
    let pairs = [("block.east", "block.flat"), ("block.flat", "block.north")];
    fillet(&block, &pairs, 1., "round").map_err(e)
}

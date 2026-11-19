//! The block from (−15, −15, 0) to (15, 15, 20), bored through by a hole of radius 6, the hole's
//! wall drafted 3° about the floor so the hole widens toward the top.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::draft::draft_up;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([-15., -15., 0.]), Vec3::new(30., 30., 20.), "block").map_err(e)?;
    let bore = cylinder(&at([0., 0., -1.]), 6., 22., "hole").map_err(e)?;
    let bored = difference(&b, &bore).map_err(e)?;
    // a positive draft narrows the part along the pull, so a bore's cone widens upward
    draft_up(&bored, &["hole.side"], 0., 3.)
}

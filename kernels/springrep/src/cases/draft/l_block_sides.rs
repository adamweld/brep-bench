//! An L-shaped block 12 tall, its six sides drafted 6° inward about its floor, the two at the
//! concave edge too.
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::draft::draft_up;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let outline: Vec<Vec2> = [(0., 0.), (30., 0.), (30., 10.), (10., 10.), (10., 25.), (0., 25.)]
        .iter()
        .map(|&(x, y)| Vec2::new(x, y))
        .collect();
    let sides = ["s0", "s1", "s2", "s3", "s4", "s5"];
    let l = extrude(&Profile::polygon(&outline, &sides), &at([0., 0., 0.]), 12., "l").map_err(e)?;
    draft_up(&l, &["l.s0", "l.s1", "l.s2", "l.s3", "l.s4", "l.s5"], 0., 6.)
}

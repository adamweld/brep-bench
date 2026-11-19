//! An L-shaped block 8 tall with arms 5 wide, its six sides drafted 30° about its floor until each
//! arm's sides meet in a ridge under the top: seven faces.
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::draft::draft_up;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let outline: Vec<Vec2> = [(0., 0.), (20., 0.), (20., 5.), (5., 5.), (5., 15.), (0., 15.)]
        .iter()
        .map(|&(x, y)| Vec2::new(x, y))
        .collect();
    let sides = ["s0", "s1", "s2", "s3", "s4", "s5"];
    let l = extrude(&Profile::polygon(&outline, &sides), &at([0., 0., 0.]), 8., "l").map_err(e)?;
    draft_up(&l, &["l.s0", "l.s1", "l.s2", "l.s3", "l.s4", "l.s5"], 0., 30.)
}

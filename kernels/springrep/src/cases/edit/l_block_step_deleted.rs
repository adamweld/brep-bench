//! An L-shaped block 8 tall; its step, the riser x = 5 and the tread y = 5, then deleted together,
//! the far end and the far side running on to meet: the box from the origin to (20, 15, 8).
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::edit::delete_named;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let outline: Vec<Vec2> = [(0., 0.), (20., 0.), (20., 5.), (5., 5.), (5., 15.), (0., 15.)]
        .iter()
        .map(|&(x, y)| Vec2::new(x, y))
        .collect();
    let sides = ["front", "end", "tread", "riser", "back", "side"];
    let l = extrude(&Profile::polygon(&outline, &sides), &at([0., 0., 0.]), 8., "l").map_err(e)?;
    delete_named(&l, &["l.tread", "l.riser"])
}

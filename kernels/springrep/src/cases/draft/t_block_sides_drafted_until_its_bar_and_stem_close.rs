//! A T-shaped block 8 tall, its eight sides drafted 45° about its floor until the bar's sides meet
//! in a ridge at height 2 and the stem's at height 3.
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::draft::draft_up;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let outline: Vec<Vec2> = [
        (0., 0.),
        (20., 0.),
        (20., 4.),
        (13., 4.),
        (13., 16.),
        (7., 16.),
        (7., 4.),
        (0., 4.),
    ]
    .iter()
    .map(|&(x, y)| Vec2::new(x, y))
    .collect();
    let sides = ["s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7"];
    let t = extrude(&Profile::polygon(&outline, &sides), &at([0., 0., 0.]), 8., "t").map_err(e)?;
    let names: Vec<String> = sides.iter().map(|s| format!("t.{s}")).collect();
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    draft_up(&t, &names, 0., 45.)
}

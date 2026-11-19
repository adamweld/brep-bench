//! A capsule's end, the cylinder of radius 5 from z = 0 to z = 10 capped by a half ball, shelled inward
//! to a wall 1 thick, open at its base.
use std::f64::consts::FRAC_PI_2;

use springrep::edit::Wall;
use springrep::topo::Brep;

use crate::shared::shell::{arc, line, shelled, turned_whole};

pub fn build() -> Result<Brep, String> {
    // the meridian: the base, the side up to z = 10, the dome's quarter circle about (0, 10) to the
    // axis, and down the axis
    let capsule = turned_whole(
        vec![
            line([0., 0.], [5., 0.], "base"),
            line([5., 0.], [5., 10.], "side"),
            arc([0., 10.], 5., 0., FRAC_PI_2, "dome"),
            line([0., 15.], [0., 0.], "axis"),
        ],
        "capsule",
    )?;
    shelled(&capsule, &["capsule.base"], 1., Wall::Inward, "shell")
}

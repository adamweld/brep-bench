//! The upper half of the ball of radius 5 about the origin, its rim rounded to radius 1, hollowed
//! to a wall 2 thick with no opening. The round moved inward turns inside out and drops: the cavity
//! is the part of the ball of radius 3 above z = 2.
use std::f64::consts::FRAC_PI_2;

use springrep::blend::fillet;
use springrep::edit::Wall;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::shell::{arc, line, shelled, turned_whole};

pub fn build() -> Result<Brep, String> {
    let half = turned_whole(
        vec![
            line([0., 0.], [5., 0.], "flat"),
            arc([0., 0.], 5., 0., FRAC_PI_2, "dome"),
            line([0., 5.], [0., 0.], "axis"),
        ],
        "half",
    )?;
    let rounded = fillet(&half, &[("half.dome", "half.flat")], 1., "round").map_err(e)?;
    shelled(&rounded, &[], 2., Wall::Inward, "shell")
}

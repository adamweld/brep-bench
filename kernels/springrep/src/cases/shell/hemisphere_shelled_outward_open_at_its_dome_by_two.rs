//! The upper half of the ball of radius 5 about the origin shelled outward to a wall 2. thick, open
//! at its dome: the slice of the ball between z = -2. and z = 0.
use std::f64::consts::FRAC_PI_2;

use springrep::edit::Wall;
use springrep::topo::Brep;

use crate::shared::shell::{arc, line, shelled, turned_whole};

pub fn build() -> Result<Brep, String> {
    // the meridian: across the flat face, up the dome's quarter circle, down the axis
    let half = turned_whole(
        vec![
            line([0., 0.], [5., 0.], "flat"),
            arc([0., 0.], 5., 0., FRAC_PI_2, "dome"),
            line([0., 5.], [0., 0.], "axis"),
        ],
        "half",
    )?;
    // the flat face moved 2. out is the wall's outside, run on until it meets the dome's sphere
    shelled(&half, &["half.dome"], 2., Wall::Outward, "shell")
}

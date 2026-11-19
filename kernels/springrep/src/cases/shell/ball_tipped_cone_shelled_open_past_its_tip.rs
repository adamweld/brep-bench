//! A cone of base radius 6 and height 12 on the z axis, its tip rounded by a ball of radius 1,
//! shelled inward to a wall 1.5 thick, open at its base. The tip's ball moved 1.5 inward is less than
//! nothing: the moved side runs on to its own apex.
use std::f64::consts::FRAC_PI_2;

use springrep::edit::Wall;
use springrep::topo::Brep;

use crate::shared::shell::{arc, line, shelled, turned_whole};

pub fn build() -> Result<Brep, String> {
    // The cone 2r + z = 12 with its tip rounded by the ball of radius 1 tangent to its side: the
    // ball's center is on the axis at z = 12 - sqrt 5, and it touches the side where the side's
    // normal, (2, 1)/sqrt 5 in (r, z), points from the center, at (2, 1)/sqrt 5 from it.
    let s5 = 5f64.sqrt();
    let c = 12. - s5;
    let touch = [2. / s5, c + 1. / s5];
    let cone = turned_whole(
        vec![
            line([0., 0.], [6., 0.], "base"),
            line([6., 0.], touch, "side"),
            arc([0., c], 1., 0.5f64.atan(), FRAC_PI_2 - 0.5f64.atan(), "tip"),
            line([0., c + 1.], [0., 0.], "axis"),
        ],
        "cone",
    )?;
    shelled(&cone, &["cone.base"], 1.5, Wall::Inward, "shell")
}

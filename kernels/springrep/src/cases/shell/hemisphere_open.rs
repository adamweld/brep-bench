//! The lower half of the ball of radius 5 about the origin, shelled inward to a wall 1 thick, open at
//! its flat face, z = 0: a bowl.
use std::f64::consts::FRAC_PI_2;

use springrep::edit::Wall;
use springrep::topo::Brep;

use crate::shared::shell::{arc, line, shelled, turned_whole};

pub fn build() -> Result<Brep, String> {
    // the meridian: the sphere's arc from the south pole up to the plane z = 0., across the
    // plane to the axis, and down the axis
    let z: f64 = 0.;
    let r = (25. - z * z).sqrt();
    let ball = turned_whole(
        vec![
            arc([0., 0.], 5., -FRAC_PI_2, FRAC_PI_2 + z.atan2(r), "sphere"),
            line([r, z], [0., z], "flat"),
            line([0., z], [0., -5.], "axis"),
        ],
        "ball",
    )?;
    shelled(&ball, &["ball.flat"], 1., Wall::Inward, "shell")
}

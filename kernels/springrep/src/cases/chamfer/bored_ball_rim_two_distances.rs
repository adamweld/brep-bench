//! The ball of radius 5 about the origin bored through along the z axis with a hole of radius 2,
//! the upper rim of the bore chamfered with two distances, 2 down the bore and 1 on the ball.
//!
//! springrep chamfers a circular edge only between faces that are straight in section (planes,
//! cylinders and cones about its axis): a sphere's meridian is a circle, and it refuses the
//! chamfer ("unsupported: a chamfer on a straight edge of a cylinder"). So the chamfer is made as
//! what it is: the line through the two points the distances reach, measured as chords in the
//! plane through the axis, turned about the axis into a cone, and the ring between the edge and
//! that cone cut away with a boolean.
use springrep::math::Vec2;
use springrep::topo::Brep;

use crate::shared::chamfer::{along_sphere, cut_by_chamfer_cone, toward};
use springrep::boolean::difference;
use springrep::primitive::{cylinder, sphere};

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let body = {
        let ball = sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)?;
        let bore = cylinder(&at([0., 0., -6.]), 2., 12., "bore").map_err(e)?;
        difference(&ball, &bore).map_err(e)?
    };
    let top = 21f64.sqrt();
    let rim = Vec2::new(2., top);
    // 2 down the bore, and 1 out over the ball as a chord
    let p = Vec2::new(2., top - 2.);
    let q = along_sphere(Vec2::ZERO, rim, 1., false);
    cut_by_chamfer_cone(&body, p, toward(p, q), Vec2::new(-1., 0.), rim)
}

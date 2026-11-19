//! The ball of radius 5 about the origin cut flat at z = 3, the rim of its flat face (the disk of
//! radius 4) chamfered with two distances: 4 across the flat face, all the way to its center, and
//! 1 on the ball, so the flat face shrinks to a point.
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
use springrep::math::Vec3;
use springrep::primitive::{block, sphere};

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let body = {
        let ball = sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)?;
        // everything above z = 3 goes; the block's floor becomes the flat face
        let above = block(&at([-10., -10., 3.]), Vec3::new(20., 20., 10.), "cut").map_err(e)?;
        difference(&ball, &above).map_err(e)?
    };
    let rim = Vec2::new(4., 3.);
    // 4 in across the flat face, to its center, and 1 down the ball as a chord
    let p = Vec2::new(0., 3.);
    let q = along_sphere(Vec2::ZERO, rim, 1., false);
    cut_by_chamfer_cone(&body, p, toward(p, q), Vec2::new(0., 1.), rim)
}

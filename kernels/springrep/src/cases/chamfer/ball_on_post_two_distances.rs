//! A post of radius 3 joined to a ball of radius 5 about (0, 0, 10), the concave circle where the
//! post leaves the ball chamfered with two distances, 2 down the post and 1 on the ball, filling
//! the groove.
//!
//! springrep chamfers a circular edge only between faces that are straight in section (planes,
//! cylinders and cones about its axis): a sphere's meridian is a circle, and it refuses the
//! chamfer ("unsupported: a chamfer on a straight edge of a cylinder"). So the chamfer is made as
//! what it is: the line through the two points the distances reach, measured as chords in the
//! plane through the axis, turned about the axis into a cone, and the ring between the edge and
//! that cone added with a boolean.
use springrep::boolean::union;
use springrep::math::Vec2;
use springrep::primitive::{cylinder, sphere};
use springrep::topo::Brep;

use crate::shared::chamfer::{along_sphere, toward, turned_polygon};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let post = cylinder(&at([0., 0., 0.]), 3., 10., "post").map_err(e)?;
    let ball = sphere(&at([0., 0., 10.]), 5., "ball").map_err(e)?;
    let body = union(&post, &ball).map_err(e)?;
    let edge = Vec2::new(3., 6.);
    // 2 down the post, and 1 out over the ball as a chord
    let p = Vec2::new(3., 4.);
    let q = along_sphere(Vec2::new(0., 10.), edge, 1., true);
    // the ring under the cone: on past q into the ball, then in to the axis and back out through
    // the post, so beside the groove it lies inside the body
    let f = q + toward(p, q) * 0.5;
    let ring = turned_polygon(&[p, f, Vec2::new(0., f.y), Vec2::new(0., p.y)], "chamfer")?;
    union(&body, &ring).map_err(e)
}

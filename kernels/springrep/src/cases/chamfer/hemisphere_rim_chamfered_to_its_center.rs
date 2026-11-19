//! The upper half of the ball of radius 5 about the origin, the rim of its flat face chamfered
//! with two distances: 5 across the flat face, all the way to its center, and 1 on the ball, so
//! the flat face shrinks to a point and the solid is a spherical sector.
//!
//! springrep chamfers a circular edge only between faces that are straight in section (planes,
//! cylinders and cones about its axis): a sphere's meridian is a circle, and it refuses the
//! chamfer ("unsupported: a chamfer on a straight edge of a cylinder"). So the chamfer is made as
//! what it is: the line through the two points the distances reach, measured as chords in the
//! plane through the axis, turned about the axis into a cone, and the ring between the edge and
//! that cone cut away with a boolean.
use springrep::math::Vec2;
use springrep::topo::Brep;

use crate::shared::chamfer::{along_sphere, cut_by_chamfer_cone, toward, hemisphere};

pub fn build() -> Result<Brep, String> {
    let body = hemisphere()?;
    let rim = Vec2::new(5., 0.);
    // 5 in across the flat face, to its center, and 1 up the ball as a chord
    let p = Vec2::new(0., 0.);
    let q = along_sphere(Vec2::ZERO, rim, 1., true);
    cut_by_chamfer_cone(&body, p, toward(p, q), Vec2::new(0., -1.), rim)
}

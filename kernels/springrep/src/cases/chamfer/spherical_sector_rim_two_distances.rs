//! The spherical sector of the ball of radius 5 inside the cone of half angle α, cos α = 3/5, its
//! rim chamfered with two distances, 2 down the cone and 1 on the ball.
//!
//! springrep chamfers a circular edge only between faces that are straight in section (planes,
//! cylinders and cones about its axis): a sphere's meridian is a circle, and it refuses the
//! chamfer ("unsupported: a chamfer on a straight edge of a cylinder"). So the chamfer is made as
//! what it is: the line through the two points the distances reach, measured as chords in the
//! plane through the axis, turned about the axis into a cone, and the ring between the edge and
//! that cone cut away with a boolean.
use springrep::math::Vec2;
use springrep::topo::Brep;

use crate::shared::chamfer::{along_sphere, cut_by_chamfer_cone, toward, sector};

pub fn build() -> Result<Brep, String> {
    let body = sector(3. / 5.)?;
    let rim = Vec2::new(4., 3.);
    // 2 down the cone, and 1 up the ball as a chord
    let p = Vec2::new(2.4, 1.8);
    let q = along_sphere(Vec2::ZERO, rim, 1., true);
    cut_by_chamfer_cone(&body, p, toward(p, q), Vec2::new(0.6, -0.8), rim)
}

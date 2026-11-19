//! The lens common to the balls of radius 5 about (0, 0, 3) and (0, 0, −3), its rim chamfered with
//! two distances, 2 on its upper face and 1 on its lower.
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
use springrep::boolean::intersection;
use springrep::primitive::sphere;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let body = {
        // the upper face is the ball below's, the lower face the ball above's
        let below = sphere(&at([0., 0., -3.]), 5., "below").map_err(e)?;
        let above = sphere(&at([0., 0., 3.]), 5., "above").map_err(e)?;
        intersection(&below, &above).map_err(e)?
    };
    let rim = Vec2::new(4., 0.);
    let (low, high) = (Vec2::new(0., -3.), Vec2::new(0., 3.));
    // 2 up the upper face and 1 down the lower, each as a chord
    let p = along_sphere(low, rim, 2., true);
    let q = along_sphere(high, rim, 1., false);
    cut_by_chamfer_cone(&body, p, toward(p, q), toward(low, p), rim)
}

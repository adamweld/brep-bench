//! The cap of the ball of radius 5 about the origin above the plane z = 4.9999: a disk 0.063 across
//! under a dome 10⁻⁴ high, around the pole.
use springrep::boolean::intersection;
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::tangent::between;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the ball's poles on the z axis: the cap's rim runs around the top one
    let ball = sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)?;
    // the half-space above z = 4.9999, past the ball on every side but its floor
    let above = between([-6., -6., 4.9999], [6., 6., 6.], "above")?;
    intersection(&ball, &above).map_err(e)
}

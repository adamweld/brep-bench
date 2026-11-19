//! The ball of radius 10 about the origin and the cylinder of radius 5 about the line x = 5, y = 0
//! from z = -12 to 12, which touches the sphere from inside at (10, 0, 0) and meets it in Viviani's
//! curve: their union.
use springrep::boolean::union;
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::tangent::rod;
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the ball's poles on the y axis and its seam through (-10, 0, 0), off the curve
    let ball = sphere(&frame([0., 0., 0.], [0., 1., 0.], [-1., 0., 0.])?, 10., "ball").map_err(e)?;
    // the cylinder's seam on the line x = 0, y = 0, away from (10, 0, 0)
    let bore = rod([5., 0., -12.], [0., 0., 1.], [-1., 0., 0.], 5., 24., "cylinder")?;
    union(&ball, &bore).map_err(e)
}

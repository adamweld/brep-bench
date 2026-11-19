//! The union of the ball of radius 0.005 about the origin and the ball of radius 0.004 about
//! (0.002, 0.003, 0.006), 0.007 from it.
use springrep::boolean::union;
use springrep::topo::Brep;

use crate::shared::boolean::ball;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let a = ball([0., 0., 0.], 0.005, "a")?;
    let b = ball([0.002, 0.003, 0.006], 0.004, "b")?;
    union(&a, &b).map_err(e)
}

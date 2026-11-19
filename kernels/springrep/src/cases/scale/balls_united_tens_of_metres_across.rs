//! The union of the ball of radius 50000 about the origin and the ball of radius 40000 about
//! (20000, 30000, 60000), 70000 from it.
use springrep::boolean::union;
use springrep::topo::Brep;

use crate::shared::boolean::ball;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let a = ball([0., 0., 0.], 50000., "a")?;
    let b = ball([20000., 30000., 60000.], 40000., "b")?;
    union(&a, &b).map_err(e)
}

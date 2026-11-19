//! The union of the ball of radius 5 about (600000, −800000, 0) and the ball of radius 4 about
//! (600002, −799997, 6), 7 from it along (2, 3, 6)/7: a solid a few units across, a million from
//! the origin.
use springrep::boolean::union;
use springrep::topo::Brep;

use crate::shared::boolean::ball;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let a = ball([600000., -800000., 0.], 5., "a")?;
    let b = ball([600002., -799997., 6.], 4., "b")?;
    union(&a, &b).map_err(e)
}

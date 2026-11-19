//! Twelve balls of radius 3 centred evenly on the circle of radius 10 in the xy plane, the first at
//! (10, 0, 0), joined one after another into a ring.
use springrep::boolean::union;
use springrep::topo::Brep;

use crate::shared::boolean::ball;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let centre = |k: usize| {
        let a = (30. * k as f64).to_radians();
        [10. * a.cos(), 10. * a.sin(), 0.]
    };
    let mut ring = ball(centre(0), 3., "ball")?;
    for k in 1..12 {
        ring = union(&ring, &ball(centre(k), 3., "ball")?).map_err(e)?;
    }
    Ok(ring)
}

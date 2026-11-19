//! The union of two balls of radius 5, about the origin and about (6, 0, 0).
use springrep::boolean::union;
use springrep::topo::Brep;

use crate::shared::boolean::ball;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let a = ball([0., 0., 0.], 5., "a")?;
    let b = ball([6., 0., 0.], 5., "b")?;
    union(&a, &b).map_err(e)
}

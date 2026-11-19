//! The block [-10, 10]²×[-10, 0] with the ball of radius 5 about the origin cut from it: a
//! hemispherical pocket in its top face.
use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::boolean::{ball, between};
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let blank = between([-10., -10., -10.], [10., 10., 0.], "block")?;
    let tool = ball([0., 0., 0.], 5., "ball")?;
    difference(&blank, &tool).map_err(e)
}

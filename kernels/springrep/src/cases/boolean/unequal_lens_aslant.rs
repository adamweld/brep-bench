//! The lens common to the ball of radius 5.511 about the origin and the ball of radius 3.317 about
//! (4.231, 3.467, 2.653): the circle their spheres meet in lies aslant to every axis.
use springrep::boolean::intersection;
use springrep::topo::Brep;

use crate::shared::boolean::ball;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let a = ball([0., 0., 0.], 5.511, "a")?;
    let b = ball([4.231, 3.467, 2.653], 3.317, "b")?;
    intersection(&a, &b).map_err(e)
}

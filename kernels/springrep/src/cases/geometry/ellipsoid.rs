//! The solid ellipsoid x²/36 + y²/16 + z²/9 ≤ 1: the unit ball stretched by 6, 4 and 3 along the
//! axes. springrep has no ellipsoid, but its affine map carries the ball's sphere onto the exact
//! rational image of it.
use springrep::edit::{Affine, affine};
use springrep::math::Vec3;
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&at([0., 0., 0.]), 1., "ball").map_err(e)?;
    let stretch = Affine {
        m: [[6., 0., 0.], [0., 4., 0.], [0., 0., 3.]],
        t: Vec3::ZERO,
    };
    affine(&ball, &stretch).map_err(e)
}

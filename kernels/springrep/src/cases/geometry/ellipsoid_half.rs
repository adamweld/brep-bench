//! The upper half of the ellipsoid x²/36 + y²/16 + z²/9 ≤ 1: the unit ball stretched by 6, 4 and 3
//! (springrep has no ellipsoid), then cut by the plane z = 0, keeping what lies above it.
use springrep::boolean::split;
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
    let ellipsoid = affine(&ball, &stretch).map_err(e)?;
    // the part on the plane normal's side comes first
    let (above, _) = split(&ellipsoid, Vec3::ZERO, Vec3::Z).map_err(e)?;
    Ok(above)
}

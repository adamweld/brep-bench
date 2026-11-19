//! A cylinder of radius 5 from z = 0 to 10 whose side is one rational B-spline surface, the rim
//! of its top rounded to radius 1 by a rolling ball.
use springrep::blend::fillet;
use springrep::loft::{Section, loft};
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // a ruled loft between two circles: springrep carries each as the rational quadratic circle
    // of nine control points from the frame's x axis (the seam on y = 0, x ≥ 0), and the side is
    // their B-spline surface, of degree 1 along z
    let circle = Profile::circle(Vec2::ZERO, 10.);
    let sections = [
        Section { profile: circle.clone(), frame: at([0., 0., 0.]) },
        Section { profile: circle, frame: at([0., 0., 10.]) },
    ];
    let body = loft(&sections, true, "cyl").map_err(e)?;
    fillet(&body, &[("cyl.end", "cyl.side")], 1., "round").map_err(e)
}

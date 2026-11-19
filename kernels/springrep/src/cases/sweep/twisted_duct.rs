//! A ruled duct along y from the square A B C D at y = 0 to the diamond A′ B′ C′ D′ at y = 40,
//! each corner running straight to its partner.
use springrep::loft::{Section, loft};
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // both sections in planes square to y, sketch x along world x and sketch y along world −z
    let plane = |y: f64| frame([0., y, 0.], [0., 1., 0.], [1., 0., 0.]);
    let corners = |pts: [(f64, f64); 4]| -> Vec<Vec2> {
        pts.iter().map(|&(x, z)| Vec2::new(x, -z)).collect()
    };
    let square = corners([(10., -10.), (-10., -10.), (-10., 10.), (10., 10.)]);
    let diamond = corners([(0., -12.), (-12., 0.), (0., 12.), (12., 0.)]);
    let sections = [
        Section { profile: Profile::polygon(&square, &["side"]), frame: plane(0.)? },
        Section { profile: Profile::polygon(&diamond, &["side"]), frame: plane(40.)? },
    ];
    loft(&sections, true, "duct").map_err(e)
}

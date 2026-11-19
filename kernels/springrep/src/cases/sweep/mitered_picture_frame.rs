//! A picture frame: a molding 8 wide and 6 tall with a 3 × 2 rabbet swept round the rectangle
//! (±30, ±20) in the xy plane, mitered at its corners.
use springrep::boolean::union;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // springrep sweeps along legs with round joints at sharp turns, not miters. A section of
    // straight sides carried round a rectangle and mitered at 45° fills, at each height, the ring
    // between the rectangle offset by the section's inner and outer reach: below the rabbet's
    // top (z < 2) from s = 3 to 8, above it from s = 0 to 8. So the frame is two extruded rings.
    let ring = |inner_s: f64| {
        let hole = Profile::rect(Vec2::ZERO, 60. + 2. * inner_s, 40. + 2. * inner_s).loops;
        Profile::rect(Vec2::ZERO, 76., 56.).with_holes(hole)
    };
    let lower = extrude(&ring(3.), &at([0., 0., 0.]), 2., "rabbet").map_err(e)?;
    let upper = extrude(&ring(0.), &at([0., 0., 2.]), 4., "molding").map_err(e)?;
    union(&lower, &upper).map_err(e)
}

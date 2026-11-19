//! A prism from z = 0 to z = 10 on the closed uniform quadratic B-spline whose control polygon is
//! the square of corners (10, 10), (−10, 10), (−10, −10), (10, −10): four parabolic arcs from the
//! middle of each side to the next. springrep's only periodic B-spline is a cubic, so the closed
//! curve is drawn as two open pieces of it on its own uniform knots (`closed_quadratic`).
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::geometry::closed_quadratic;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let corners = [[10., 10.], [-10., 10.], [-10., -10.], [10., -10.]];
    let section = Profile::outline(closed_quadratic(corners, "side")?);
    extrude(&section, &at([0., 0., 0.]), 10., "prism").map_err(e)
}

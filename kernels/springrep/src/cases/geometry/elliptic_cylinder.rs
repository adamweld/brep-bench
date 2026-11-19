//! The ellipse x²/25 + y²/9 ≤ 1 in the plane z = 0, extruded straight up to z = 10.
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::geometry::ellipse;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let section = Profile::outline(ellipse(Vec2::ZERO, 5., 3., "side"));
    extrude(&section, &at([0., 0., 0.]), 10., "drum").map_err(e)
}

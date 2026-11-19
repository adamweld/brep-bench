//! The block −12 ≤ x ≤ 12, −8 ≤ y ≤ 8, 0 ≤ z ≤ 8 with the elliptic hole x²/36 + y²/9 < 1 bored
//! through it along z, cut by the ellipse's extrusion running past both faces.
use springrep::boolean::difference;
use springrep::math::{Vec2, Vec3};
use springrep::primitive::block;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::geometry::ellipse;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = block(&at([-12., -8., 0.]), Vec3::new(24., 16., 8.), "plate").map_err(e)?;
    let section = Profile::outline(ellipse(Vec2::ZERO, 6., 3., "wall"));
    let bore = extrude(&section, &at([0., 0., -1.]), 10., "bore").map_err(e)?;
    difference(&plate, &bore).map_err(e)
}

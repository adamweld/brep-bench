//! An L-shaped outline in the xy plane extruded 8 along +z.
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let corners = [(0., 0.), (20., 0.), (20., 5.), (5., 5.), (5., 15.), (0., 15.)];
    let points: Vec<Vec2> = corners.iter().map(|&(x, y)| Vec2::new(x, y)).collect();
    let outline = Profile::polygon(&points, &["side"]);
    extrude(&outline, &at([0., 0., 0.]), 8., "plate").map_err(e)
}

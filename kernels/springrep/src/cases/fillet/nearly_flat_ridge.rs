//! A prism 6 high on the pentagon (0, 0), (8, 0), (8, 5), (4, 5 + 4 tan 0.5°), (0, 5): a block
//! whose top is two faces meeting at 179° along a ridge at x = 4, the ridge rounded to radius 180.
use springrep::blend::fillet;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let peak = 5. + 4. * 0.5f64.to_radians().tan();
    let outline = [(0., 0.), (8., 0.), (8., 5.), (4., peak), (0., 5.)];
    let outline = outline.map(|(x, y)| Vec2::new(x, y));
    let names = ["bottom", "east", "roof_east", "roof_west", "west"];
    let section = Profile::polygon(&outline, &names);
    let prism = extrude(&section, &at([0., 0., 0.]), 6., "prism").map_err(e)?;
    fillet(&prism, &[("prism.roof_east", "prism.roof_west")], 180., "round").map_err(e)
}

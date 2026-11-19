//! A wedge 6 high on the triangle (0, 0), (10, 0), (10, 10 tan 5°): its edge along the z axis,
//! where two faces meet at 5°, rounded to radius 0.1.
use springrep::blend::fillet;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let rise = 10. * 5f64.to_radians().tan();
    let outline = [(0., 0.), (10., 0.), (10., rise)].map(|(x, y)| Vec2::new(x, y));
    let section = Profile::polygon(&outline, &["floor", "heel", "blade"]);
    let wedge = extrude(&section, &at([0., 0., 0.]), 6., "wedge").map_err(e)?;
    // the knife edge is where the floor y = 0 meets the blade's slope, on the z axis
    fillet(&wedge, &[("wedge.floor", "wedge.blade")], 0.1, "round").map_err(e)
}

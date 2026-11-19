//! The block from the origin to (40, 30, 15) with a pocket from (8, 8, 5) to (32, 22, 15) cut into
//! its top, the pocket's four walls drafted 5° about its floor so it widens toward its opening.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::draft::draft_up;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(40., 30., 15.), "block").map_err(e)?;
    // the cutter runs out past the top, so the pocket is open there
    let cutter = block(&at([8., 8., 5.]), Vec3::new(24., 14., 11.), "pocket").map_err(e)?;
    let part = difference(&b, &cutter).map_err(e)?;
    // the walls' outward normals face the pocket's middle; a positive draft tilts them away
    // from it as they rise
    let walls = ["pocket.u_min", "pocket.u_max", "pocket.v_min", "pocket.v_max"];
    draft_up(&part, &walls, 5., 5.)
}

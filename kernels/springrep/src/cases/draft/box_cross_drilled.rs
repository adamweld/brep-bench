//! The box from the origin to (30, 20, 20), drilled through along x by a hole of radius 4 on the
//! line y = 10, z = 10, then the two faces the hole comes out of drafted 8° about the floor.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::draft::draft_up;
use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(30., 20., 20.), "box").map_err(e)?;
    let drill = cylinder(&frame([-1., 10., 10.], [1., 0., 0.], [0., 1., 0.])?, 4., 32., "hole")
        .map_err(e)?;
    let drilled = difference(&b, &drill).map_err(e)?;
    // the ends x = 0 and x = 30 tilt about their bottom edges; the hole's wall is trimmed to them
    draft_up(&drilled, &["box.u_min", "box.u_max"], 0., 8.)
}

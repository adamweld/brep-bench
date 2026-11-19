//! The block from the origin to (40, 30, 15) with a pocket from (8, 8, 5) to (32, 22, 15), its
//! upright corners rounded to radius 4, cut into its top; the pocket's eight walls drafted 5° about
//! its floor so it widens toward its opening, the rounds turning into quarter cones.
use springrep::blend::fillet;
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::draft::draft_up;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(40., 30., 15.), "block").map_err(e)?;
    // the cutter, its upright edges rounded, runs out past the block's top
    let cutter = block(&at([8., 8., 5.]), Vec3::new(24., 14., 11.), "pocket").map_err(e)?;
    let corners = [
        ("pocket.u_min", "pocket.v_min"),
        ("pocket.v_min", "pocket.u_max"),
        ("pocket.u_max", "pocket.v_max"),
        ("pocket.v_max", "pocket.u_min"),
    ];
    let cutter = fillet(&cutter, &corners, 4., "round").map_err(e)?;
    let part = difference(&b, &cutter).map_err(e)?;
    let walls = [
        "pocket.u_min",
        "pocket.u_max",
        "pocket.v_min",
        "pocket.v_max",
        "round.round_0",
        "round.round_1",
        "round.round_2",
        "round.round_3",
    ];
    draft_up(&part, &walls, 5., 5.)
}

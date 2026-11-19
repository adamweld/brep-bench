//! The box from the origin to (30, 20, 10), its four upright edges rounded to radius 4, then all
//! eight sides drafted 5° about the floor: the rounds turn into quarter cones about their axes.
use springrep::blend::fillet;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::draft::draft_up;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(30., 20., 10.), "box").map_err(e)?;
    let corners = [
        ("box.u_min", "box.v_min"),
        ("box.v_min", "box.u_max"),
        ("box.u_max", "box.v_max"),
        ("box.v_max", "box.u_min"),
    ];
    let rounded = fillet(&b, &corners, 4., "round").map_err(e)?;
    let sides = [
        "box.u_min",
        "box.u_max",
        "box.v_min",
        "box.v_max",
        "round.round_0",
        "round.round_1",
        "round.round_2",
        "round.round_3",
    ];
    draft_up(&rounded, &sides, 0., 5.)
}

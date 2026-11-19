//! A plate with a square boss on it, the boss's four upright edges rounded to radius 3, and then
//! its root, now one tangent chain round the rounded square, rounded to radius 2, concave.
use springrep::blend::fillet;
use springrep::boolean::union;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = block(&at([0., 0., 0.]), Vec3::new(40., 30., 5.), "plate").map_err(e)?;
    let boss = block(&at([12., 7., 5.]), Vec3::new(16., 16., 10.), "boss").map_err(e)?;
    let body = union(&plate, &boss).map_err(e)?;
    // 1. the upright edges, where neighbouring sides meet
    let sides = ["boss.v_min", "boss.u_max", "boss.v_max", "boss.u_min"];
    let upright: Vec<(&str, &str)> = (0..4).map(|i| (sides[i], sides[(i + 1) % 4])).collect();
    let body = fillet(&body, &upright, 3., "upright").map_err(e)?;
    // 2. the root: one side's edge on the plate picks the whole chain round the boss
    fillet(&body, &[("plate.end", "boss.u_min")], 2., "root").map_err(e)
}

//! An L-block, its concave inside edge rounded to radius 2, and then the edge of its end face
//! y = 0 that runs up the upright, round that round and along the foot rounded to radius 2,
//! convex: the corner where they meet is a quarter of a torus.
use springrep::blend::fillet;
use springrep::boolean::union;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let foot = block(&at([0., 0., 0.]), Vec3::new(20., 10., 6.), "foot").map_err(e)?;
    let upright = block(&at([0., 0., 0.]), Vec3::new(6., 10., 20.), "up").map_err(e)?;
    let body = union(&foot, &upright).map_err(e)?;
    // 1. the concave edge where the foot's top meets the upright's face x = 6
    let body = fillet(&body, &[("foot.end", "up.u_max")], 2., "inside").map_err(e)?;
    // 2. the end face's edge up the upright, which now runs on tangentially round the inside
    //    round and along the foot: one chain, rounded whole
    fillet(&body, &[("up.v_min", "up.u_max")], 2., "outside").map_err(e)
}

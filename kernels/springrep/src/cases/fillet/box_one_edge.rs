//! A cube of side 10 from the origin, the edge where its faces x = 10 and y = 10 meet rounded to
//! radius 2.
use springrep::blend::fillet;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let cube = block(&at([0., 0., 0.]), Vec3::new(10., 10., 10.), "cube").map_err(e)?;
    // a block's side x = max is `u_max` and its side y = max `v_max`
    fillet(&cube, &[("cube.u_max", "cube.v_max")], 2., "round").map_err(e)
}

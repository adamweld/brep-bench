//! A cube of side 10 with all twelve edges chamfered 2, one after another, each chamfer ending on
//! the faces at its ends (the chamfers made before it among them), so the three at each corner
//! meet in a point. The four round the top go in one operation, mitred where they meet, then the
//! four round the bottom, then the four upright edges, which end on those chamfers.
use springrep::blend::chamfer;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let cube = block(&at([0., 0., 0.]), Vec3::new(10., 10., 10.), "cube").map_err(e)?;
    let sides = ["cube.v_min", "cube.u_max", "cube.v_max", "cube.u_min"];
    let round = |cap: &'static str| sides.map(|s| (cap, s));
    let cube = chamfer(&cube, &round("cube.end"), 2., "top").map_err(e)?;
    let cube = chamfer(&cube, &round("cube.start"), 2., "bottom").map_err(e)?;
    let upright: Vec<(&str, &str)> = (0..4).map(|i| (sides[i], sides[(i + 1) % 4])).collect();
    chamfer(&cube, &upright, 2., "upright").map_err(e)
}

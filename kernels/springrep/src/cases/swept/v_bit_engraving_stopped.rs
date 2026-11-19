//! The block (0, −10, −10)–(30, 10, 0) less everything a V bit passes through as it slides along
//! the x axis by 15: the cone about the upright line through (5, 0), pointed at (5, 0, −2), opening
//! at 90° to a flat top of radius 5 at z = 3.
//!
//! springrep has no sweep of a solid. Every line along x through the bit crosses its section by the
//! plane x = 5 through its axis, a triangle, so the bit sweeps itself where it starts and stops and
//! that triangle carried along 15, a prism (`shared::swept`, a slide).
use springrep::boolean::difference;
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::swept::{boxed, polygon, shifted, slid, unite, v3};
use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    let block = boxed([0.0, -10.0, -10.0], [30.0, 10.0, 0.0], "block")?;
    let bit = cone(&at([5.0, 0.0, -2.0]), 0.0, 5.0, 5.0, "bit").map_err(e)?;
    // the triangle in (y, z) on the plane x = 5
    let section = polygon(&[(0.0, -2.0), (5.0, 3.0), (-5.0, 3.0)], "flank");
    let plane = frame([5.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])?;
    let v = v3(15.0, 0.0, 0.0);
    let middle = slid(&section, &plane, v, "groove")?;
    let groove = unite(&[&bit, &middle, &shifted(&bit, v)])?;
    difference(&block, &groove).map_err(e)
}

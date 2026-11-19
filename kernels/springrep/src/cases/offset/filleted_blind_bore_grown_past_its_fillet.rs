//! The cylinder of radius 10 from z = 0 to z = 8 with a blind bore of radius 5 down to a floor at
//! z = 2, the concave edge round the floor filleted to radius 2, grown by 3: the fillet vanishes
//! and the moved bore side and floor meet in a sharp concave edge. springrep 1.0.1 refuses it: its
//! offset cannot follow the concave fillet moved past its radius ("an offset … closes a torus to
//! nothing").
use springrep::blend::fillet;
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let block = cylinder(&at([0., 0., 0.]), 10., 8., "block").map_err(e)?;
    let bore = cylinder(&at([0., 0., 2.]), 5., 7., "bore").map_err(e)?;
    let bored = difference(&block, &bore).map_err(e)?;
    let filleted = fillet(&bored, &[("bore.start", "bore.side")], 2., "floor").map_err(e)?;
    grown(&filleted, 3.)
}

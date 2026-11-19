//! The plate from (−10, −10, 0) to (10, 10, 4) with a hole of radius 3 drilled through it along
//! the z axis, grown by 4, past the hole's radius: the hole fills, and each end of it leaves a
//! dimple, a spindle torus's inside, its lowest point on the axis.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = block(&at([-10., -10., 0.]), Vec3::new(20., 20., 4.), "plate").map_err(e)?;
    let drill = cylinder(&at([0., 0., -1.]), 3., 6., "hole").map_err(e)?;
    let drilled = difference(&plate, &drill).map_err(e)?;
    grown(&drilled, 4.)
}

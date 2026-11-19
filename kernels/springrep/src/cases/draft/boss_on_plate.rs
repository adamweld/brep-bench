//! A 40 × 40 × 5 plate with a boss of radius 8 standing 12 on it, the boss's side drafted 4° about
//! the plate's top, z = 5: a frustum of a cone on the plate.
use springrep::boolean::union;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::draft::draft_up;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = block(&at([-20., -20., 0.]), Vec3::new(40., 40., 5.), "plate").map_err(e)?;
    let boss = cylinder(&at([0., 0., 5.]), 8., 12., "boss").map_err(e)?;
    let part = union(&plate, &boss).map_err(e)?;
    // the boss's cylinder turns into the cone through its foot circle on the neutral plane
    draft_up(&part, &["boss.side"], 5., 4.)
}

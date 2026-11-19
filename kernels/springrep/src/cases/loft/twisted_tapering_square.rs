//! A square column from z = 0 to 40 of side 30 − z/2, turned 4.5° per unit of height about the
//! z axis: from the square of side 30 to the square of side 10, half a turn on.
use springrep::loft::{Section, loft};
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // no loft along guide curves: a smooth loft through the level sections, the square at each
    // height turned and shrunk as the case gives it
    let sections = (0..=40)
        .map(|i| {
            let z = i as f64;
            let a = (4.5 * z).to_radians();
            let plane = frame([0., 0., z], [0., 0., 1.], [a.cos(), a.sin(), 0.])?;
            let side = 30. - z / 2.;
            Ok(Section { profile: Profile::rect(Vec2::ZERO, side, side), frame: plane })
        })
        .collect::<Result<Vec<_>, String>>()?;
    loft(&sections, false, "column").map_err(e)
}

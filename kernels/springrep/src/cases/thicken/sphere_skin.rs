//! The sphere of radius 5 about the origin, a closed sheet, thickened by 1 outward: the hollow ball
//! between radii 5 and 6.
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::thicken::thickened;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the sheet is the skin of a ball
    let ball = sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)?;
    thickened(&ball, &["ball.skin"], 1.)
}

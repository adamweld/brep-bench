//! A cube of side 20 with the cube from (10, 10, 10) to (20, 20, 20) notched out of its corner, the
//! notch's three concave edges rounded to radius 3 at once, the inside corner closed by a ball.
use springrep::blend::fillet;
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let cube = block(&at([0., 0., 0.]), Vec3::new(20., 20., 20.), "cube").map_err(e)?;
    let notch = block(&at([10., 10., 10.]), Vec3::new(10., 10., 10.), "notch").map_err(e)?;
    let body = difference(&cube, &notch).map_err(e)?;
    // the notch's inside faces are its block's faces x = 10, y = 10 and z = 10
    let pairs = [
        ("notch.u_min", "notch.v_min"),
        ("notch.v_min", "notch.start"),
        ("notch.start", "notch.u_min"),
    ];
    fillet(&body, &pairs, 3., "round").map_err(e)
}

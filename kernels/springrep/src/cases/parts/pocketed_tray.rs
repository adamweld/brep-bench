//! An open tray: the block (−36, 0, −24)–(36, 16, 24) less the pocket (−33, 0, −21)–(33, 13, 21),
//! with four tubes (radius 5, bore 2) standing from y = 5 to the floor at y = 13.
use springrep::boolean::{difference, union_all};
use springrep::math::{Vec2, Vec3};
use springrep::primitive::block;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    let blank = block(&at([-36., 0., -24.]), Vec3::new(72., 16., 48.), "tray").map_err(e)?;
    let pocket = block(&at([-33., 0., -21.]), Vec3::new(66., 13., 42.), "pocket").map_err(e)?;
    let tray = difference(&blank, &pocket).map_err(e)?;
    // each standoff a ring extruded along +y from y = 5 onto the floor; its bore stops there
    let along_y = |x: f64, z: f64| frame([x, 5., z], [0., 1., 0.], [1., 0., 0.]);
    let ring = Profile::circle(Vec2::ZERO, 10.).with_holes(Profile::circle(Vec2::ZERO, 4.).loops);
    let mut parts = vec![tray];
    for (x, z) in [(26., 14.), (-26., 14.), (-26., -14.), (26., -14.)] {
        parts.push(extrude(&ring, &along_y(x, z)?, 8., "standoff").map_err(e)?);
    }
    union_all(&parts.iter().collect::<Vec<_>>()).map_err(e)
}

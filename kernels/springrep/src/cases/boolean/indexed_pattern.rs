//! A disk of radius 20 from z = -5 to 0 with six holes of radius 2 through it along z, their axes on
//! the circle of radius 12, the first at (12, 0) and the rest turned by 60° steps, each its own cut.
use springrep::boolean::difference;
use springrep::math::{Transform, Vec3};
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let mut disk = cylinder(&at([0., 0., -5.]), 20., 5., "disk").map_err(e)?;
    // the first hole's drill, running 1 past both faces
    let first = cylinder(&at([12., 0., -6.]), 2., 7., "hole").map_err(e)?;
    for k in 0..6 {
        let mut drill = first.clone();
        let turn = (60. * k as f64).to_radians();
        drill.transform(&Transform::rotation(Vec3::ZERO, Vec3::Z, turn));
        disk = difference(&disk, &drill).map_err(e)?;
    }
    Ok(disk)
}

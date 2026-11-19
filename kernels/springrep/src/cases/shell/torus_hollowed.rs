//! A solid torus about the z axis, its tube of radius 2 about a middle circle of radius 6, hollowed
//! to a wall 0.5 thick with no opening: the torus less the torus of tube radius 1.5.
use springrep::edit::Wall;
use springrep::primitive::torus;
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ring = torus(&at([0., 0., 0.]), 6., 2., "ring").map_err(e)?;
    shelled(&ring, &[], 0.5, Wall::Inward, "shell")
}

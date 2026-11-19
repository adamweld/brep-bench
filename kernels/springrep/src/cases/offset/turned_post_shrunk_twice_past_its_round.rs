//! The cylinder of radius 5 from z = 0 to z = 6 topped by the frustum from radius 5 to radius 2 at
//! z = 10, its shoulder rounded to radius 1, shrunk by 2: the round vanishes and the moved
//! cylinder and cone meet in a sharp edge.
//! springrep 1.0.1 refuses it: its face offset cannot follow a round moved past its radius ("an
//! offset … closes a torus to nothing").
use springrep::topo::Brep;

use crate::shared::offset::{shrunk, turned_post};

pub fn build() -> Result<Brep, String> {
    shrunk(&turned_post()?, 2.)
}

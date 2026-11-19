//! The solid the box (0, 0, 0)–(10, 6, 4) fills as it slides without turning along (8, 5, 3).
//!
//! springrep has no sweep of a solid. A point the box passes through lies in the box where it
//! starts, or behind one of the faces the box leads with, which it has carried along the slide; so
//! the sweep is the box and those three faces (x = 10, y = 6, z = 4) carried along (8, 5, 3), three
//! oblique prisms (`shared::swept::slid_cuboid`).
use springrep::math::Vec3;
use springrep::topo::Brep;

use crate::shared::swept::{slid_cuboid, v3};

pub fn build() -> Result<Brep, String> {
    slid_cuboid(v3(5.0, 3.0, 2.0), [Vec3::X, Vec3::Y, Vec3::Z], [5.0, 3.0, 2.0], v3(8.0, 5.0, 3.0), "box")
}

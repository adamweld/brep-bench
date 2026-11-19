//! The solid a ring fills as it slides 15 along x: the torus about the z axis, its tube's centers
//! 6 from it and its tube of radius 2.
//!
//! springrep has no sweep of a solid. The torus is everything within 2 of its tube's center
//! circle, and the slide, longer than the hole is wide, carries that circle over a stadium: so the
//! sweep is everything within 2 of the stadium, its middle the stadium's section square to the
//! slide carried along, and its ends half the ring with its hole filled
//! (`shared::swept::slid_ring`).
use springrep::math::Vec3;
use springrep::topo::Brep;

use crate::shared::swept::slid_ring;

pub fn build() -> Result<Brep, String> {
    slid_ring(Vec3::ZERO, Vec3::Z, Vec3::X, 6.0, 2.0, 15.0, "ring")
}

//! The rod of radius 5 about the z axis from z = 0 to z = 20 less everything a wheel of two beads
//! passes through as it slides 40 along z: two tori on the line through (11.5, 0, −9) along y, the
//! first about (11.5, 0, −9) with radii 6 and 2, the second about (11.5, 1.5, −9) with radii 6 and
//! 1.5.
//!
//! springrep has no sweep of a solid. The sweep of a union is the union of the sweeps, and each
//! torus is everything within its tube's radius of its center circle, which the slide, longer than
//! the hole is wide, carries over a stadium: its sweep is everything within that radius of the
//! stadium (`shared::swept::slid_ring`).
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{rod, slid_ring, unite, v3};

pub fn build() -> Result<Brep, String> {
    let first = slid_ring(v3(11.5, 0.0, -9.0), Vec3::Y, Vec3::Z, 6.0, 2.0, 40.0, "bead")?;
    let second = slid_ring(v3(11.5, 1.5, -9.0), Vec3::Y, Vec3::Z, 6.0, 1.5, 40.0, "bead")?;
    difference(&rod()?, &unite(&[&first, &second])?).map_err(e)
}

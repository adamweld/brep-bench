//! The rectangle 6 ≤ x ≤ 10, 0 ≤ z ≤ 4 of the xz plane turned 90° about the z axis, from +x
//! toward +y: a quarter of a washer.
use std::f64::consts::FRAC_PI_2;

use springrep::math::{Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the xz plane, its sketch x along world x and its sketch y along world z
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    let section = Profile::rect(Vec2::new(8., 2.), 4., 4.);
    revolve(&section, &xz, Vec3::ZERO, Vec3::Z, FRAC_PI_2, "sector").map_err(e)
}

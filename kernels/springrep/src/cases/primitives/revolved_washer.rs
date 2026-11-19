//! The rectangle 6 ≤ x ≤ 10, 0 ≤ z ≤ 4 of the xz plane turned a full turn about the z axis.
use std::f64::consts::TAU;

use springrep::math::{Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    let section = Profile::rect(Vec2::new(8., 2.), 4., 4.);
    revolve(&section, &xz, Vec3::ZERO, Vec3::Z, TAU, "washer").map_err(e)
}

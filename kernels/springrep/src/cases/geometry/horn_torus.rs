//! The horn torus: the disk (x − 5)² + z² ≤ 25 of the xz plane turned a whole turn about the z
//! axis, which it touches at the origin. springrep's torus primitive wants its tube clear of the
//! axis, but a revolve takes the disk touching it.
use springrep::math::{Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::revolve;
use springrep::topo::Brep;
use std::f64::consts::TAU;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the meridian disk, drawn on the xz plane (sketch y along world z), turned about z
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    let disk = Profile::circle(Vec2::new(5., 0.), 10.);
    revolve(&disk, &xz, Vec3::ZERO, Vec3::Z, TAU, "ring").map_err(e)
}

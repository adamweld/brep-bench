//! A ring about the z axis whose section, in the xz plane, is the closed uniform quadratic B-spline
//! on the square of corners (13, 3), (7, 3), (7, −3), (13, −3), turned a whole turn. springrep's
//! only periodic B-spline is a cubic, so the closed curve is drawn as two open pieces of it on its
//! own uniform knots (`closed_quadratic`).
use springrep::math::Vec3;
use springrep::profile::Profile;
use springrep::sweep::revolve;
use springrep::topo::Brep;
use std::f64::consts::TAU;

use crate::shared::geometry::closed_quadratic;
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // sketch coordinates on the xz plane: (x, z)
    let corners = [[13., 3.], [7., 3.], [7., -3.], [13., -3.]];
    let section = Profile::outline(closed_quadratic(corners, "section")?);
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    revolve(&section, &xz, Vec3::ZERO, Vec3::Z, TAU, "ring").map_err(e)
}

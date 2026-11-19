//! The spindle torus of middle radius 3 and tube radius 5, counted once: the part x ≥ 0 of the disk
//! (x − 3)² + z² ≤ 25 of the xz plane, bounded by the tube's outer arc and the z axis from (0, −4)
//! to (0, 4), turned a whole turn about the z axis.
use springrep::math::{Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::revolve;
use springrep::topo::Brep;
use std::f64::consts::TAU;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the outer arc runs from the axis below round through (8, 0) to the axis above: it meets x = 0
    // where cos t = −3/5
    let meets = (-3f64 / 5.).acos();
    let half = Profile::outline(vec![
        NamedSegment::arc(Vec2::new(3., 0.), 5., -meets, 2. * meets, "tube"),
        NamedSegment::line(Vec2::new(0., 4.), Vec2::new(0., -4.), "axis"),
    ]);
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    revolve(&half, &xz, Vec3::ZERO, Vec3::Z, TAU, "apple").map_err(e)
}

//! The paraboloid x² + y² ≤ 8z filled up to z = 8: the parabola x² = 8z of the xz plane from its
//! vertex to (8, 8), turned a whole turn about the z axis and closed by the disk at z = 8.
use springrep::math::{Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::revolve;
use springrep::topo::Brep;
use std::f64::consts::TAU;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // A parabola's arc is a quadratic Bézier: the vertex's tangent is level, the one at (8, 8) has
    // slope 2, and they meet at (4, 0).
    let side = Segment::Bezier {
        ctrl: vec![Vec2::new(0., 0.), Vec2::new(4., 0.), Vec2::new(8., 8.)],
        weights: vec![1., 1., 1.],
    };
    let meridian = Profile::outline(vec![
        NamedSegment::new(side, "side"),
        NamedSegment::line(Vec2::new(8., 8.), Vec2::new(0., 8.), "top"),
        NamedSegment::line(Vec2::new(0., 8.), Vec2::new(0., 0.), "axis"),
    ]);
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    revolve(&meridian, &xz, Vec3::ZERO, Vec3::Z, TAU, "bowl").map_err(e)
}

//! The solid inside the upper sheet of the hyperboloid z²/9 − (x² + y²)/16 = 1, below z = 6: the
//! hyperbola z²/9 − x²/16 = 1 from its vertex (0, 3) to (4√3, 6), turned a whole turn about the z
//! axis, closed by the disk at z = 6.
use springrep::math::{Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::revolve;
use springrep::topo::Brep;
use std::f64::consts::TAU;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // The arc as a rational quadratic Bézier: the tangent at the vertex is level, the one at
    // (4√3, 6) meets it at (4/√3, 3), and the weight there, √6/2, puts the arc's middle on the
    // hyperbola.
    let s3 = 3f64.sqrt();
    let side = Segment::Bezier {
        ctrl: vec![Vec2::new(0., 3.), Vec2::new(4. / s3, 3.), Vec2::new(4. * s3, 6.)],
        weights: vec![1., 6f64.sqrt() / 2., 1.],
    };
    let meridian = Profile::outline(vec![
        NamedSegment::new(side, "side"),
        NamedSegment::line(Vec2::new(4. * s3, 6.), Vec2::new(0., 6.), "top"),
        NamedSegment::line(Vec2::new(0., 6.), Vec2::new(0., 3.), "axis"),
    ]);
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    revolve(&meridian, &xz, Vec3::ZERO, Vec3::Z, TAU, "cap").map_err(e)
}

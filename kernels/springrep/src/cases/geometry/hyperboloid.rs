//! The solid inside the hyperboloid of one sheet x² + y² = 16(1 + z²/36), between z = −6 and z = 6:
//! the hyperbola x²/16 − z²/36 = 1 (its branch x > 0) turned a whole turn about the z axis, closed
//! by two flat disks.
use springrep::math::{Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::revolve;
use springrep::topo::Brep;
use std::f64::consts::{SQRT_2, TAU};

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // The hyperbola from (4√2, −6) to (4√2, 6) as a rational quadratic Bézier: its end tangents
    // meet on the waist's line at (2√2, 0), and the curve passes through the vertex (4, 0) at its
    // middle, which takes the weight √2 there.
    let r = 4. * SQRT_2;
    let side = Segment::Bezier {
        ctrl: vec![Vec2::new(r, -6.), Vec2::new(2. * SQRT_2, 0.), Vec2::new(r, 6.)],
        weights: vec![1., SQRT_2, 1.],
    };
    let meridian = Profile::outline(vec![
        NamedSegment::new(side, "side"),
        NamedSegment::line(Vec2::new(r, 6.), Vec2::new(0., 6.), "top"),
        NamedSegment::line(Vec2::new(0., 6.), Vec2::new(0., -6.), "axis"),
        NamedSegment::line(Vec2::new(0., -6.), Vec2::new(r, -6.), "bottom"),
    ]);
    // drawn on the xz plane, sketch y along world z
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    revolve(&meridian, &xz, Vec3::ZERO, Vec3::Z, TAU, "waist").map_err(e)
}

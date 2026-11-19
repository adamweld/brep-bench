//! The lemon: the circular segment of the disk of radius 5 about (−3, 0) in the xz plane on the
//! side x ≥ 0, cut off by its chord on the z axis from (0, −4) to (0, 4), turned a whole turn about
//! the z axis.
//!
//! Turned as an arc, the arc's circle would make the inner sheet of a spindle torus, whose middle
//! circle lies across the axis from the arc; springrep 1.0.1 builds that face off its own edges (by
//! 6 mm) and its checker refuses the result. So the arc is drawn as the rational quadratic Bézier
//! that is exactly the same arc, and is turned into a surface of revolution of that curve.
use springrep::math::{Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::revolve;
use springrep::topo::Brep;
use std::f64::consts::TAU;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // The arc subtends 2θ at (−3, 0), cos θ = 3/5. Its end tangents meet on the x axis at
    // −3 + 5/cos θ = 16/3, and a circular arc's middle weight is cos θ.
    let skin = Segment::Bezier {
        ctrl: vec![Vec2::new(0., -4.), Vec2::new(16. / 3., 0.), Vec2::new(0., 4.)],
        weights: vec![1., 3. / 5., 1.],
    };
    let segment = Profile::outline(vec![
        NamedSegment::new(skin, "skin"),
        NamedSegment::line(Vec2::new(0., 4.), Vec2::new(0., -4.), "axis"),
    ]);
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    revolve(&segment, &xz, Vec3::ZERO, Vec3::Z, TAU, "lemon").map_err(e)
}

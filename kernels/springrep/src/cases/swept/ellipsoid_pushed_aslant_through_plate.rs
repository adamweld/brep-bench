//! The plate (−15, −15, 0)–(15, 15, 4) less everything an ellipsoid passes through as it slides
//! along (4, 3, 12): semi-axes 3, 2 and 1.5 along x, y and z about (−2, −1.5, −4).
//!
//! springrep has no sweep of a solid. The ellipsoid is the unit ball stretched by
//! A = diag(3, 2, 1.5) and moved to its center, and a stretch carries a slide to a slide, so its
//! sweep is the unit ball's sweep along A⁻¹(4, 3, 12) = (4/3, 3/2, 8), stretched the same way. The
//! unit ball's sweep along a line is a capsule: the tube of radius 1 about the segment with a half
//! ball at each end, one outline turned about the segment. springrep's affine map carries it to the
//! ellipsoid's sweep exactly.
use std::f64::consts::{FRAC_PI_2, TAU};

use springrep::boolean::difference;
use springrep::edit::{Affine, affine};
use springrep::math::{Frame, Vec3};
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{arc, boxed, line, outline, v3};

pub fn build() -> Result<Brep, String> {
    let plate = boxed([-15.0, -15.0, 0.0], [15.0, 15.0, 4.0], "plate")?;
    let w = v3(4.0 / 3.0, 1.5, 8.0);
    let len = w.norm();
    // the capsule's half outline, sketch x out from the segment and y along it
    let half = outline(vec![
        arc((0.0, 0.0), 1.0, -FRAC_PI_2, FRAC_PI_2, "start"),
        line((1.0, 0.0), (1.0, len), "side"),
        arc((0.0, len), 1.0, 0.0, FRAC_PI_2, "end"),
        line((0.0, len + 1.0), (0.0, -1.0), "axis"),
    ]);
    let along = w * (1.0 / len);
    let x = along.any_perpendicular();
    let sketch = Frame::new(Vec3::ZERO, x.cross(along), x).map_err(e)?;
    let capsule = revolve(&half, &sketch, Vec3::ZERO, along, TAU, "capsule").map_err(e)?;
    let stretch = Affine {
        m: [[3.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 1.5]],
        t: v3(-2.0, -1.5, -4.0),
    };
    let hole = affine(&capsule, &stretch).map_err(e)?;
    difference(&plate, &hole).map_err(e)
}

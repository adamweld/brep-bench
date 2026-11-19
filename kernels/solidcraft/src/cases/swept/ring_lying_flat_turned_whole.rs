//! The solid a ring fills as it is turned a whole turn about the z axis: the torus about the
//! vertical line through (8, 0) whose tube, of radius 1, is centred on the circle of radius 3 in
//! the plane z = 0.
//!
//! solidcraft-kernel has no sweep of a solid, so the turn is built by elementary geometry: turned
//! a whole turn about an axis, a solid sweeps its shadow, the circles about the axis it meets,
//! revolved. At height z the ring is the annulus about (8, 0) between 3 ∓ √(1 − z²), which meets
//! the circles about the z axis from 5 − √(1 − z²) out to 11 + √(1 − z²): the shadow is the
//! stadium of the band |z| ≤ 1 from 5 to 11 ended by half discs of radius 1.
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::{FRAC_PI_2, PI};

pub fn build() -> R<Body> {
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    let end = |x: f64, start: f64| Seg2::Arc { center: v2(x, 0.0), radius: 1.0, start, sweep: PI };
    let stadium = vec![
        line((5.0, -1.0), (11.0, -1.0)),
        end(11.0, -FRAC_PI_2),
        line((11.0, 1.0), (5.0, 1.0)),
        end(5.0, FRAC_PI_2),
    ];
    revolve_z(Region2 { outer: Loop2 { segs: stadium }, holes: vec![] }, 360.0)
}

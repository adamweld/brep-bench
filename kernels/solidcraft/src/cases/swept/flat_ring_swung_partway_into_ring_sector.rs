//! A ring sector, everything between 8 and 12 from the z axis with |z| ≤ 1 and polar angle from
//! 0° to 60°, notched by a ring lying flat swung about the z axis from −140° to +30°: the torus
//! about the vertical line through (10, 0) whose tube, of radius 0.4, is centred on the circle of
//! radius 1 in the plane z = 0.
//!
//! solidcraft-kernel has no sweep of a solid, so the swing is built by elementary geometry
//! ([`swung_partway`]). A circle about the z axis through the ring's hole meets it in two arcs,
//! either side of the hole, a few degrees apart: far less than the turn, so on each circle the
//! ring passes over everything from its first point where it starts to its last where it stops,
//! as the ring with its hole filled does (everything within 1 + √(0.4² − z²) of its axis at
//! height z). That filled ring meets each circle in one arc holding the circle's point at 0°, so
//! its section through its axis, a stadium, is the shadow, revolved through the turn, with its
//! half ahead of that section where it stops.
use super::drum_swung_partway_into_ring_sector::{half_ahead, swung_partway};
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::{FRAC_PI_2, PI};

pub fn build() -> R<Body> {
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    // the tube's half circle on the outer side of the ring's axis (x = 11) or the inner (x = 9)
    let tube = |x: f64, start: f64| Seg2::Arc { center: v2(x, 0.0), radius: 0.4, start, sweep: PI };
    let stadium = vec![
        line((9.0, -0.4), (11.0, -0.4)),
        tube(11.0, -FRAC_PI_2),
        line((11.0, 0.4), (9.0, 0.4)),
        tube(9.0, FRAC_PI_2),
    ];
    let half = vec![
        line((10.0, -0.4), (11.0, -0.4)),
        tube(11.0, -FRAC_PI_2),
        line((11.0, 0.4), (10.0, 0.4)),
        line((10.0, 0.4), (10.0, -0.4)),
    ];
    let region = |segs: Vec<Seg2>| Region2 { outer: Loop2 { segs }, holes: vec![] };
    swung_partway(region(stadium), Some(half_ahead(region(half))?))
}

//! A ring sector, everything between 8 and 12 from the z axis with |z| ≤ 1 and polar angle from
//! 0° to 60°, notched by a bull-nose mill swung about the z axis from −140° to +30°: about the
//! vertical line through (10, 0) from z = −0.5 to 0.5, a cylinder of radius 1 whose bottom corner
//! is rounded to radius 0.3, the round's tube centred on the circle of radius 0.7 at z = −0.2.
//!
//! solidcraft-kernel has no sweep of a solid, so the swing is built by elementary geometry
//! ([`swung_partway`]): the mill meets each circle about the z axis in one arc holding the
//! circle's point at 0°, so its section through its axis there is its shadow, revolved through
//! the turn, with the mill's half ahead of that section where it stops.
use super::drum_swung_partway_into_ring_sector::{half_ahead, swung_partway};
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::FRAC_PI_2;

pub fn build() -> R<Body> {
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    // the round's quarter circle in the section, on either side of the axis at x = 10
    let round = |x: f64, start: f64| Seg2::Arc { center: v2(x, -0.2), radius: 0.3, start, sweep: FRAC_PI_2 };
    let section = vec![
        line((9.3, -0.5), (10.7, -0.5)),
        round(10.7, -FRAC_PI_2),
        line((11.0, -0.2), (11.0, 0.5)),
        line((11.0, 0.5), (9.0, 0.5)),
        line((9.0, 0.5), (9.0, -0.2)),
        round(9.3, 2.0 * FRAC_PI_2),
    ];
    let half = vec![
        line((10.0, -0.5), (10.7, -0.5)),
        round(10.7, -FRAC_PI_2),
        line((11.0, -0.2), (11.0, 0.5)),
        line((11.0, 0.5), (10.0, 0.5)),
        line((10.0, 0.5), (10.0, -0.5)),
    ];
    let region = |segs: Vec<Seg2>| Region2 { outer: Loop2 { segs }, holes: vec![] };
    swung_partway(region(section), Some(half_ahead(region(half))?))
}

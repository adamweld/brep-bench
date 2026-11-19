//! A ring sector, everything between 8 and 12 from the z axis with |z| ≤ 1 and polar angle from
//! 0° to 60°, notched by a domed mill swung about the z axis from −140° to +30°: about the
//! vertical line through (10, 0), a cylinder of radius 1 up to z = 3, ended below by the cap of
//! the sphere of radius 2 about (10, 0, 1.2), which meets the cylinder at z = 1.2 − √3.
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
    let rim = 1.2 - 3f64.sqrt(); // where the cap meets the cylinder, 30° off the cap's axis
    let cap = |start: f64, sweep: f64| Seg2::Arc { center: v2(10.0, 1.2), radius: 2.0, start, sweep };
    let sixth = FRAC_PI_2 / 3.0;
    let section = vec![
        cap(-FRAC_PI_2 - sixth, 2.0 * sixth),
        line((11.0, rim), (11.0, 3.0)),
        line((11.0, 3.0), (9.0, 3.0)),
        line((9.0, 3.0), (9.0, rim)),
    ];
    let half = vec![
        cap(-FRAC_PI_2, sixth),
        line((11.0, rim), (11.0, 3.0)),
        line((11.0, 3.0), (10.0, 3.0)),
        line((10.0, 3.0), (10.0, -0.8)),
    ];
    let region = |segs: Vec<Seg2>| Region2 { outer: Loop2 { segs }, holes: vec![] };
    swung_partway(region(section), Some(half_ahead(region(half))?))
}

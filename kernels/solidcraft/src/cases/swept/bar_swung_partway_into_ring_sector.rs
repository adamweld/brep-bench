//! A ring sector, everything between 8 and 12 from the z axis with |z| ≤ 1 and polar angle from
//! 0° to 60°, notched by a bar, the box 10 ≤ x ≤ 11, |y| ≤ 0.5, |z| ≤ 0.6, swung about the z axis
//! from −140° to +30°.
//!
//! solidcraft-kernel has no sweep of a solid, so the swing is built by elementary geometry
//! ([`swung_partway`]). Out from 10 to 11 the bar meets each circle about the z axis in one arc
//! holding its point at 0°; out from 11 to its outer corners, √(11² + 0.5²), in two arcs, beside
//! its corners, a few degrees apart: far less than the turn, so on each circle the bar passes over
//! everything from its first point where it starts to its last where it stops, as does the bar
//! filled out to the arc through its outer corners about the z axis. That meets each circle in one
//! arc holding the point at 0°, so its section there, out from 10 to √(11² + 0.5²) at |z| ≤ 0.6,
//! is the shadow, revolved through the turn; where the bar stops, its filled half ahead of the
//! section is the half of the bar there (y ≥ 0) with the sliver from x = 11 out to the arc.
use super::drum_swung_partway_into_ring_sector::swung_partway;
use crate::solids::*;
use solidcore_record::Seg2;

pub fn build() -> R<Body> {
    let corner = (11f64 * 11.0 + 0.25).sqrt();
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    // the filled half in plan, from the inner face along the section out to the arc about the
    // z axis, round it to the leading outer corner and back along the leading face
    let plan = vec![
        line((10.0, 0.0), (corner, 0.0)),
        Seg2::Arc { center: v2(0.0, 0.0), radius: corner, start: 0.0, sweep: 0.5f64.atan2(11.0) },
        line((11.0, 0.5), (10.0, 0.5)),
        line((10.0, 0.5), (10.0, 0.0)),
    ];
    let level = Plane { origin: Vec3::ZERO, x: Vec3::X, y: Vec3::Y };
    let lead = extrude(&level, Region2 { outer: Loop2 { segs: plan }, holes: vec![] }, -0.6, 0.6)?;
    swung_partway(rect(10.0, -0.6, corner, 0.6), Some(lead))
}

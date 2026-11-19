//! A ring sector, everything between 8 and 12 from the z axis with |z| ≤ 1 and polar angle from
//! 0° to 60°, slotted by a pin, the solid cylinder of radius 0.5 about the x axis from x = 6 to
//! 11, swung about the z axis from −140° to +140°.
//!
//! solidcraft-kernel has no sweep of a solid, so the swing is built by elementary geometry. The
//! pin spans a few degrees about the z axis, starts and ends clear of the block, and turns far
//! enough that every circle about the z axis in the block that meets the pin is passed over
//! where it crosses the block: what the pin takes from the block is its shadow, the circles it
//! meets, revolved. At height z the pin is the rectangle 6 ≤ x ≤ 11, |y| ≤ √(0.5² − z²), nearest
//! the axis at 6 and farthest at its outer corners, √(11² + 0.5² − z²): the shadow is the band
//! |z| ≤ 0.5 from 6 out to the circle of radius √(11² + 0.5²) about the origin.
use super::drum_swung_partway_into_ring_sector::{ring_sector, turned};
use crate::solids::*;
use solidcore_record::Seg2;

pub fn build() -> R<Body> {
    let far = (11f64 * 11.0 + 0.25).sqrt();
    let reach = 0.5f64.atan2(11.0); // the far circle's angle at z = ±0.5
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    let shadow = vec![
        line((6.0, -0.5), (11.0, -0.5)),
        Seg2::Arc { center: v2(0.0, 0.0), radius: far, start: -reach, sweep: 2.0 * reach },
        line((11.0, 0.5), (6.0, 0.5)),
        line((6.0, 0.5), (6.0, -0.5)),
    ];
    let shadow = Region2 { outer: Loop2 { segs: shadow }, holes: vec![] };
    let swept = turned(&revolve_z(shadow, 280.0)?, -140.0)?;
    cut(&ring_sector()?, &swept)
}

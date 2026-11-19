//! A block, the box from (−10, −10, −5) to (10, 10, 0), with a pocket cut by an end mill turned
//! 120° about the z axis, right-handed: the solid cylinder of radius 3 about the vertical line
//! through (1.5, 0) from z = −2 to 4, whose flat end the z axis passes through.
//!
//! solidcraft-kernel has no sweep of a solid, so the turn is built by elementary geometry. The
//! mill is a disc about (1.5, 0) at every height, which holds each circle about the z axis out to
//! 1.5 whole and meets each farther one, out to 4.5, in one arc holding its point at 0°. So it
//! passes over such an arc where it starts, the 120° on from there and the arc where it stops:
//! the sweep is its section by the half-plane at 0°, out from the axis to 4.5, revolved through
//! the turn, with the mill's half behind that half-plane's plane where it starts (y ≤ 0) and its
//! half ahead of it where it stops (each holding what the turn misses of the circles it holds
//! whole).
use super::drum_swung_partway_into_ring_sector::turned;
use crate::solids::*;

pub fn build() -> R<Body> {
    let block = block((-10.0, -10.0, -5.0), (10.0, 10.0, 0.0))?;
    let mill = zcyl(1.5, 0.0, 3.0, -2.0, 4.0)?;
    let half = |positive: bool| -> R<Body> {
        let plane = Plane { origin: Vec3::ZERO, x: Vec3::Z, y: Vec3::X }; // normal +y
        solidcraft_kernel::half_space(&mill, &plane, positive)?
            .0
            .ok_or_else(|| KernelError::Failed("the mill's half is empty".into()))
    };
    let fan = revolve_z(rect(0.0, -2.0, 4.5, 4.0), 120.0)?;
    let swept = union(&union(&half(false)?, &fan)?, &turned(&half(true)?, 120.0)?)?;
    cut(&block, &swept)
}

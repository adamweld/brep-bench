//! A plate, the box from (0, −5, 0) to (20, 5, 4), with a pocket cut into its underside by a drum,
//! the solid cylinder of radius 2 about the line y = 0, z = −5 from x = 6 to x = 14, slid without
//! turning by 8 along (0.3, 0, √0.91), stopping inside the plate.
//!
//! solidcraft-kernel has no sweep of a solid, so the drum's sweep is built by elementary geometry,
//! from its halves either side of the plane z = −5 through its axis. A solid slid sweeps itself
//! where it starts and its faces that lead the slide (outward normal along it) slid the whole
//! way; or, as well, itself where it stops and its faces that trail the slide, slid. The lower
//! half's curved side trails, so it sweeps itself and its leading faces, the rectangle on z = −5
//! and its half end on x = 14; the upper half's curved side leads, so it sweeps itself where it
//! stops and its trailing faces, the same rectangle and its half end on x = 6. All three faces are
//! flat, and the kernel's `sweep` along a straight path translates each. The lower half where the
//! drum starts lies below z = −5, clear of the plate; the rest is what the sweep takes from it.
use crate::solids::*;
use solidcore_record::Seg2;
use solidcraft_kernel::{PathSeg, sweep};
use std::f64::consts::PI;

/// The half of the disc of radius 2 about the origin from angle `start` (radians) round half a
/// turn, closed by its diameter.
fn half_disc(start: f64) -> Region2 {
    let arc = Seg2::Arc { center: Vec2::ZERO, radius: 2.0, start, sweep: PI };
    let diameter = Seg2::Line { a: arc.end(), b: arc.start() };
    Region2 { outer: Loop2 { segs: vec![arc, diameter] }, holes: vec![] }
}

pub fn build() -> R<Body> {
    let plate = block((0.0, -5.0, 0.0), (20.0, 5.0, 4.0))?;
    let slide = v3(0.3, 0.0, 0.91f64.sqrt()) * 8.0;
    // the upper half of the drum where it stops
    let axis = v3(6.0, 0.0, -5.0) + slide;
    let drum = solidcraft_kernel::cylinder(axis, Vec3::X, 2.0, 8.0)?;
    let up = Plane::from_normal(axis, Vec3::Z).ok_or_else(|| KernelError::Invalid("up".into()))?;
    let mut swept = solidcraft_kernel::half_space(&drum, &up, true)?
        .0
        .ok_or_else(|| KernelError::Failed("the half drum is empty".into()))?;
    // the faces slid: the rectangle through the axis, the lower half end on x = 14 and the upper
    // half end on x = 6, each end drawn in (y, z) about the axis
    let end = |x: f64| Plane { origin: v3(x, 0.0, -5.0), x: Vec3::Y, y: Vec3::Z };
    let faces = [
        (Plane { origin: v3(6.0, -2.0, -5.0), x: Vec3::X, y: Vec3::Y }, rect(0.0, 0.0, 8.0, 4.0)),
        (end(14.0), half_disc(PI)),
        (end(6.0), half_disc(0.0)),
    ];
    for (plane, face) in &faces {
        let path = [PathSeg::Line { a: plane.origin, b: plane.origin + slide }];
        swept = union(&swept, &sweep(plane, face, &path)?)?;
    }
    cut(&plate, &swept)
}

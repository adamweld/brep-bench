//! The solid a rod fills as it is spun 120° about the z axis, right-handed: the solid cylinder of
//! radius 0.5 about the x axis from x = −3 to 5, which the z axis crosses.
//!
//! solidcraft-kernel has no sweep of a solid, so the turn is built by elementary geometry. At
//! height z the rod is the rectangle −3 ≤ x ≤ 5, |y| ≤ w, w = √(0.5² − z²). It holds each circle
//! about the z axis out to w whole, and meets each farther one in an arc about its point at 0°,
//! out to the arm's end at 5, or about its point at 180°, out to 3; and beyond the ends, out to
//! the far corners, √(5² + w²) and √(3² + w²), in two arcs beside the corners, a few degrees
//! apart. That gap is far narrower than the turn, so the rod passes over all the rod filled out
//! to the spheres about the origin through its end rims does: K′, the rod with its ends
//! capped by the spheres of radii √(5² + 0.5²) and √(3² + 0.5²) instead of flat, which meets every
//! circle in one arc about its point at 0° or at 180°, or holds it whole. Turned by 120°, an arc
//! about 0° is passed over where K′ starts, the 120° on from 0°, and where it stops; one about
//! 180°, likewise from 180°. So the sweep is K′'s section by the half-plane at 0° revolved from 0°
//! to 120°, its section by the half-plane at 180° revolved from 180° to 300°, and K′ where it
//! starts and where it stops, each by the quarters beyond those half-planes: behind them where it
//! starts, ahead of them where it stops (holding what the turn misses of the circles K′ holds
//! whole). Joined on their flat faces, the quarters meet the revolved sections only there.
use super::drum_swung_partway_into_ring_sector::turned;
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::{PI, TAU};

/// The arm's section in x and z, out from the axis to the sphere through its end rim at `end`.
fn arm(end: f64) -> Region2 {
    let far = (end * end + 0.25).sqrt();
    let reach = 0.5f64.atan2(end);
    let segs = vec![
        Seg2::Line { a: v2(0.0, -0.5), b: v2(end, -0.5) },
        Seg2::Arc { center: v2(0.0, 0.0), radius: far, start: -reach, sweep: 2.0 * reach },
        Seg2::Line { a: v2(end, 0.5), b: v2(0.0, 0.5) },
        Seg2::Line { a: v2(0.0, 0.5), b: v2(0.0, -0.5) },
    ];
    Region2 { outer: Loop2 { segs }, holes: vec![] }
}

/// The quarter of `body` on the given sides of the planes x = 0 and y = 0.
fn quarter(body: &Body, x_positive: bool, y_positive: bool) -> R<Body> {
    let part = |b: &Body, plane: Plane, positive| -> R<Body> {
        solidcraft_kernel::half_space(b, &plane, positive)?
            .0
            .ok_or_else(|| KernelError::Failed("the rod's quarter is empty".into()))
    };
    let across_x = Plane { origin: Vec3::ZERO, x: Vec3::Y, y: Vec3::Z }; // normal +x
    let across_y = Plane { origin: Vec3::ZERO, x: Vec3::Z, y: Vec3::X }; // normal +y
    part(&part(body, across_x, x_positive)?, across_y, y_positive)
}

pub fn build() -> R<Body> {
    // K′, revolved about the x axis: its section above the axis in x and z, from the far end of
    // the arm along +x round to the far end of the other
    let (long, short) = ((25.25f64).sqrt(), (9.25f64).sqrt());
    let section = vec![
        Seg2::Line { a: v2(-short, 0.0), b: v2(long, 0.0) },
        Seg2::Arc { center: v2(0.0, 0.0), radius: long, start: 0.0, sweep: 0.5f64.atan2(5.0) },
        Seg2::Line { a: v2(5.0, 0.5), b: v2(-3.0, 0.5) },
        Seg2::Arc { center: v2(0.0, 0.0), radius: short, start: PI - 0.5f64.atan2(3.0), sweep: 0.5f64.atan2(3.0) },
    ];
    let section = Region2 { outer: Loop2 { segs: section }, holes: vec![] };
    let filled = one(solidcraft_kernel::revolve(&xz(), &[section], Vec2::ZERO, Vec2::X, TAU)?)?;
    // the sections by the half-planes at 0° and at 180° (drawn at 0° and turned half a turn)
    let ahead_of_0 = revolve_z(arm(5.0), 120.0)?;
    let ahead_of_180 = turned(&revolve_z(arm(3.0), 120.0)?, 180.0)?;
    let pieces = [
        quarter(&filled, true, false)?,
        ahead_of_0,
        quarter(&filled, false, true)?,
        ahead_of_180,
        turned(&quarter(&filled, true, true)?, 120.0)?,
        turned(&quarter(&filled, false, false)?, 120.0)?,
    ];
    let mut swept = pieces[0].clone();
    for piece in &pieces[1..] {
        swept = union(&swept, piece)?;
    }
    Ok(swept)
}

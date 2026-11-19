//! The solid a ball of radius 2 about (1, 0, 0) fills as it is swung 120° about the z axis,
//! right-handed, which passes through it.
//!
//! solidcraft-kernel has no sweep of a solid, so the swing is built by elementary geometry. The
//! ball meets each circle about the z axis in one arc about its point at 0° (the points nearer
//! the ball's centre than 2), or holds it whole, so it passes over such an arc where it starts,
//! the 120° on from there and the arc where it stops. The sweep is the ball's section by the
//! half-plane at 0°, the half of its great disc in the plane y = 0 out from the axis, revolved
//! through the turn, with the ball's half behind that plane where it starts (y ≤ 0) and its half
//! ahead of it where it stops (each holding what the turn misses of the circles the ball holds
//! whole). Joined on their flat faces, the halves meet the revolved disc only there, not
//! tangentially along its rim, as whole balls would.
use super::ball_swung_groove::half_ball;
use super::drum_swung_partway_into_ring_sector::turned;
use crate::solids::*;
use solidcore_record::Seg2;

pub fn build() -> R<Body> {
    // the great disc's half out from the axis: the arc from (0, −√3) round through (3, 0) to
    // (0, √3), and the axis back down
    let s3 = 3f64.sqrt();
    let reach = s3.atan2(-1.0); // the arc's ends' angle about the centre, (0, √3)
    let section = vec![
        Seg2::Arc { center: v2(1.0, 0.0), radius: 2.0, start: -reach, sweep: 2.0 * reach },
        Seg2::Line { a: v2(0.0, s3), b: v2(0.0, -s3) },
    ];
    let fan = revolve_z(Region2 { outer: Loop2 { segs: section }, holes: vec![] }, 120.0)?;
    let behind = half_ball(v3(1.0, 0.0, 0.0), -Vec3::Y, 2.0)?;
    let ahead = turned(&half_ball(v3(1.0, 0.0, 0.0), Vec3::Y, 2.0)?, 120.0)?;
    union(&union(&behind, &fan)?, &ahead)
}

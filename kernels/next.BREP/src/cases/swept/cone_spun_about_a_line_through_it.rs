//! The solid a cone fills as it is spun 150° about the z axis, which crosses its axis inside it.
//!
//! The cone's axis (through the origin along w = (cos 30°, 0, sin 30°)) and the z axis meet at the
//! origin, so every point of the base's rim lies at one distance from it, R = √(5² + (7 tan 25°)²).
//! Fill the cone out to the ball of radius R about the origin: K′ = the cone with its flat base
//! replaced by the cap of that sphere. A circle about the z axis stays on that sphere, and it meets
//! the cone in two arcs mirrored in the xz-plane (with neither the +x nor the −x direction between
//! them) only beyond the base, where the cap fills the gap between them: the gap is narrower than
//! the turn, so K′ sweeps what the cone sweeps. K′ meets every circle about the z axis in arcs that
//! hold the direction 0° or 180° (it is symmetric in the xz-plane and holds the circle's points
//! there), and an arc [a, b] holding 0° turned by up to 150° covers [a, b + 150°] = [a, b] ∪
//! [0°, 150°] ∪ [a + 150°, b + 150°]. So the sweep is K′ where it starts, K′ where it stops, and the
//! halves of K′'s section by the xz-plane at x ≥ 0 and x ≤ 0 revolved about the z axis through
//! 0°…150° and 180°…330°: cones, spheres and planes, united.
use crate::solids::sweep::{moved, turn_z};
use crate::solids::swept::{planar, Piece};
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, revolve_profile_brep, BooleanOperation, BrepSolid, Vec3};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (s30, c30) = 30f64.to_radians().sin_cos();
    let w = p(c30, 0.0, s30);
    let u = p(-s30, 0.0, c30); // square to w, in the xz-plane
    let rb = 7.0 * 25f64.to_radians().tan(); // the base's radius
    let big = (25.0 + rb * rb).sqrt(); // the rim's distance from the origin
    let rim_angle = 5f64.atan2(rb);
    // K′: about w, (radial, axial) from the origin: the point at axial −2, the side out to the
    // rim at (rb, 5), the sphere's arc up to the axis at R.
    let filled = planar(
        &[
            Piece::Line { from: [0.0, -2.0], to: [rb, 5.0] },
            Piece::Arc { centre: [0.0, 0.0], radius: big, start: rim_angle, end: FRAC_PI_2 },
            Piece::Line { from: [0.0, big], to: [0.0, -2.0] },
        ],
        p(0.0, 0.0, 0.0),
        u,
        w,
    )?;
    let start = revolve_profile_brep(&filled, p(0.0, 0.0, 0.0), w, TAU)?;
    let turn = 150f64.to_radians();
    let end = moved(&start, turn_z(turn, p(0.0, 0.0, 0.0))?)?;
    // K′'s section by the xz-plane, in (x, z): the point A = −2w, the rims r± = 5w ± rb·u, the
    // sphere's arc between them through Rw. The z axis crosses the two sides at P+ (upper) and P−.
    let at = |s: f64, side: f64| -> Vec3 { w.scale(s).add(u.scale(side)) };
    let a = at(-2.0, 0.0);
    let (rim_up, rim_down) = (at(5.0, rb), at(5.0, -rb));
    let cross = |q: Vec3| -> Vec3 {
        // where the segment from A to q meets x = 0
        let k = -a.x / (q.x - a.x);
        a.add(q.sub(a).scale(k))
    };
    let (top, bottom) = (cross(rim_up), cross(rim_down));
    // Angles in the xz-plane (from +x toward +z) of the rims on the sphere about the origin.
    let angle = |q: Vec3| q.z.atan2(q.x);
    let outer = planar(
        &[
            Piece::Line { from: [bottom.x, bottom.z], to: [rim_down.x, rim_down.z] },
            Piece::Arc { centre: [0.0, 0.0], radius: big, start: angle(rim_down), end: angle(rim_up) },
            Piece::Line { from: [rim_up.x, rim_up.z], to: [top.x, top.z] },
            Piece::Line { from: [top.x, top.z], to: [bottom.x, bottom.z] },
        ],
        p(0.0, 0.0, 0.0),
        p(1.0, 0.0, 0.0),
        p(0.0, 0.0, 1.0),
    )?;
    let ahead = revolve_profile_brep(&outer, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), turn)?;
    let inner = polygon(&[a, bottom, top])?;
    // The x ≤ 0 half sits at 180°: revolved through 150° from there.
    let behind = revolve_profile_brep(&inner, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), turn)?;
    // United revolved parts first: the kernel's n-ary union refuses the same four given the cone
    // first (an open loop where the parts touch along the generators in the xz-plane).
    Ok(boolean_operation_nary(&[ahead, behind, end, start], BooleanOperation::Union)?)
}

//! The solid the rod of radius 0.5 about the x axis from x = −3 to 5 fills as it turns 120° about
//! the z axis.
//!
//! springrep has no sweep of a solid. A circle about the z axis meets the rod in an arc across the
//! plane y = 0 on each side of the axis, or lies wholly in it, except beyond its ends: there, past
//! x = 5 and within √(5² + 0.5²) of the origin (past x = −3 and within √(3² + 0.5²)), it meets the
//! rod in two short arcs either side of y = 0, which the turn carries across the gap between them.
//! So the rod sweeps what the rod with those gaps filled does, out to the spheres about the origin
//! through its end rims; that solid meets every circle about the z axis in arcs across y = 0. The
//! sweep is it where it starts and stops and its sections by the half-planes y = 0, x ≥ 0 and
//! x ≤ 0, turned through 120° (`shared::swept`, a turn).
use std::f64::consts::PI;

use springrep::boolean::intersection;
use springrep::math::Vec2;
use springrep::primitive::sphere;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::swept::{arc, deg, line, outline, swing, turned, unite};
use crate::shared::{at, e, frame};

/// The rod's section by the half-plane at `angle`, `length` long, capped by the sphere through its
/// end rim.
fn section(length: f64) -> Profile {
    let reach = (length * length + 0.25).sqrt();
    let a = 0.5f64.atan2(length);
    outline(vec![
        line((0.0, -0.5), (length, -0.5), "side"),
        arc((0.0, 0.0), reach, -a, 2.0 * a, "end"),
        line((length, 0.5), (0.0, 0.5), "side"),
        line((0.0, 0.5), (0.0, -0.5), "axis"),
    ])
}

pub fn build() -> Result<Brep, String> {
    // the rod, long enough to reach both spheres, cut by them
    let disc = Profile::circle(Vec2::ZERO, 1.0);
    let long = extrude(&disc, &frame([-3.1, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])?, 8.2, "rod")
        .map_err(e)?;
    let ahead = sphere(&at([0.0, 0.0, 0.0]), (25.25f64).sqrt(), "end").map_err(e)?;
    let behind = sphere(&at([0.0, 0.0, 0.0]), (9.25f64).sqrt(), "end").map_err(e)?;
    let front = intersection(&long, &ahead).map_err(e)?;
    let back = intersection(&long, &behind).map_err(e)?;
    let (front, back) = (
        springrep::boolean::half_space(&front, springrep::math::Vec3::ZERO, springrep::math::Vec3::X)
            .and_then(|h| intersection(&front, &h))
            .map_err(e)?,
        springrep::boolean::half_space(&back, springrep::math::Vec3::ZERO, -springrep::math::Vec3::X)
            .and_then(|h| intersection(&back, &h))
            .map_err(e)?,
    );
    let filled = unite(&[&front, &back])?;
    let by = deg(120.0);
    let ahead = swing(&section(5.0), 0.0, by, "swept")?;
    let behind = swing(&section(3.0), PI, by, "swept")?;
    unite(&[&filled, &ahead, &behind, &turned(&filled, by)])
}

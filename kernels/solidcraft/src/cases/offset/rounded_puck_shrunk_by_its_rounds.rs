//! A puck, the cylinder of radius 6 on the z axis from z = 0 to z = 10 with both rims rounded to
//! radius 2, shrunk by 2, the rounds' radius: every point at least 2 from its outside, the
//! kernel's offset inward with the moved faces that cross trimmed and merged.
use crate::bench::{edge_through, reloaded};
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{blend, make_cylinder, Blend, Body, Corners};
use std::f64::consts::TAU;

/// `body` with its circles about the z axis, each `(radius, z)`, rounded to `size`: every edge
/// on each circle, picked at three points round it (a circle may be split at seams).
pub fn round_circles(body: Body, circles: &[(f64, f64)], size: f64) -> Result<Body, Refusal> {
    let (body, order) = reloaded(&body)?;
    let mut edges = Vec::new();
    for &(r, z) in circles {
        for k in 0..3 {
            let a = 1.0 + k as f64 * TAU / 3.0;
            let e = edge_through(&body, &order, [r * a.cos(), r * a.sin(), z])?;
            if !edges.contains(&e) {
                edges.push(e);
            }
        }
    }
    blend(&body, Blend::Round, &edges, &[size], &[], Corners::Filled)?;
    Ok(body)
}

/// The puck: the cylinder with its rims, the circles of radius 6 at z = 0 and z = 10, rounded.
pub fn rounded_puck() -> Result<Body, Refusal> {
    round_circles(make_cylinder([0.0, 0.0, 0.0], 6.0, 10.0)?, &[(6.0, 0.0), (6.0, 10.0)], 2.0)
}

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&rounded_puck()?, -2.0)
}

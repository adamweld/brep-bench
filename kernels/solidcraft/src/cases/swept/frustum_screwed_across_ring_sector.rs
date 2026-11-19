//! A sector of a ring about the z axis (radii 8 to 12, z from −1 to 1, polar angles 0° to 60°)
//! with a helical groove milled by a frustum about the upright line through (10, 0), of radius 0.4
//! at z = −0.6 and 1 at z = 1.4, carried along the screw about the z axis that rises 2 a turn,
//! from a turn of −140° to one of +140°.
//!
//! No layer has a sweep of a solid, so the cutter's sweep is built by elementary geometry from
//! the screw sweep of planar regions, on solidcore-ops (`screw_sweep`, held to the runner's
//! `SCREW_TOLERANCE`), since solidcraft-kernel's `sweep_helix` holds its walls only to a
//! ten-thousandth of the lead, twice as loose here. The cone widens at
//! α, tan α = 0.3, from its apex A = (10, 0, −0.6 − 0.4/tan α). On its line at angle ψ about its
//! axis (ψ = 0 along +x) the outward normal is (cos α cos ψ, cos α sin ψ, −sin α), and a point
//! there moves with (−y, x, k), k = 2/2π, per radian of the screw: the two are square where
//! 10 cos α sin ψ = k sin α, on two whole lines through A, at ψ and 180° − ψ, sin ψ = k tan α/10.
//! They span the plane through A along x and along (0, sin α sin ψ, cos α), which cuts the cutter
//! in a trapezoid. Split by that plane, the half towards +y moves away from a point only through
//! its bottom and the cut, so a point it passes over and leaves before the end is left through
//! one of those; the other half likewise first reaches a point through its top or the cut. So the
//! cutter's sweep is the cutter where it starts and where it ends, clear of the block, with the
//! screw sweeps of the trapezoid and the two ends, each of which crosses the motion, never moving
//! along itself.
use super::square_bar_screwed_through_rod::screw_ops;
use crate::bench as ops;
use crate::solids::*;
use crate::Refusal;
use std::f64::consts::TAU;

pub fn build() -> Result<solidcore_ops::Body, Refusal> {
    let (lead, start, stop) = (2.0, -140.0, 140.0);
    let (low, high, r_low, r_high): (f64, f64, f64, f64) = (-0.6, 1.4, 0.4, 1.0);
    let alpha = ((r_high - r_low) / (high - low)).atan();
    let apex = v3(10.0, 0.0, low - r_low / alpha.tan());
    let sin_psi = lead / TAU * alpha.tan() / 10.0;
    let cos_psi = (1.0 - sin_psi * sin_psi).sqrt();
    // the contact plane: sketch x along +x, sketch y up the plane, midway between the two lines
    let up = v3(0.0, alpha.sin() * sin_psi, alpha.cos());
    let up_len = up.len();
    let plane = Plane { origin: apex, x: Vec3::X, y: up * (1.0 / up_len) };
    // the point b up the plane is b cos α / |up| above the apex, and the lines there are
    // b sin α cos ψ / |up| either side of it
    let along = |z: f64| (z - apex.z) * up_len / alpha.cos();
    let half = alpha.sin() * cos_psi / up_len;
    let (b0, b1) = (along(low), along(high));
    let at = |a: f64, b: f64| {
        let p = plane.origin + plane.x * a + plane.y * b;
        [p.x, p.y, p.z]
    };
    let trapezoid = ops::polygon(&[at(-half * b0, b0), at(half * b0, b0), at(half * b1, b1), at(-half * b1, b1)]);
    let section = ops::polygon(&[[8.0, 0.0, -1.0], [12.0, 0.0, -1.0], [12.0, 0.0, 1.0], [8.0, 0.0, 1.0]]);
    let mut block = solidcore_ops::sweep::revolve(&section, [0.0; 3], [0.0, 0.0, 1.0], 60f64.to_radians(), true)?;
    block = ops::cut(block, screw_ops(&trapezoid, lead, start, stop)?)?;
    for (z, r) in [(high, r_high), (low, r_low)] {
        let end = ops::circle([10.0, 0.0, z], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], r);
        block = ops::cut(block, screw_ops(&[end], lead, start, stop)?)?;
    }
    Ok(block)
}

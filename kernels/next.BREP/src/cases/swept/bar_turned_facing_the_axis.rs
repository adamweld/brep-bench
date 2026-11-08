//! The ring sector 8 ≤ ρ ≤ 12, −1 ≤ z ≤ 1, 0° to 60° about the z axis, less everything a leaning
//! bar passes through as it turns rigidly about the z axis from −140° to +140°: in the plane y = 0
//! at the turn's start, its inner side the plane through (10, 0, 0) square to
//! n = (cos 20°, 0, −sin 20°), the bar running 3 beyond it along n, 2 either side of y = 0, and 3
//! either way along l = (sin 20°, 0, cos 20°).
//!
//! next.BREP's sweep of a solid takes a prism only in motions square to its own direction, and the
//! bar leans to the turn's axis. By elementary geometry: a point at distance ρ from the z axis, at
//! height z and at angle φ from the bar's mid-plane is in the bar only if its distance beyond the
//! inner side, (ρ cos φ − 10) cos 20° − z sin 20°, is at least 0, so only if
//! ρ ≥ ρ cos φ ≥ 10 + z tan 20°. Every point of the block with ρ ≥ 10 + z tan 20° is in the bar when
//! the turn brings the bar's mid-plane to its angle (φ = 0, which the turn passes for every angle
//! of the block): there it is (ρ − 10) cos 20° − z sin 20° ≤ 2.22 < 3 beyond the inner side,
//! |(ρ − 10) sin 20° + z cos 20°| ≤ 1.63 < 3 along l and 0 from y = 0. So the block loses exactly
//! its part beyond the cone ρ = 10 + z tan 20°: the kernel's revolve about the z axis of the region
//! beyond that line in the half plane y = 0, cut from the block.
use crate::solids::p;
use crate::solids::fillet::subtract;
use crate::solids::{polygon, swept::ring_sector};
use crate::Refusal;
use brep_kernel::{revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = ring_sector(8.0, 12.0, -1.0, 1.0, 0.0, 60f64.to_radians())?;
    let lean = 20f64.to_radians().tan();
    let beyond = polygon(&[p(10.0 - 1.5 * lean, 0.0, -1.5), p(13.0, 0.0, -1.5), p(13.0, 0.0, 1.5),
        p(10.0 + 1.5 * lean, 0.0, 1.5)])?;
    let cut = revolve_profile_brep(&beyond, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)?;
    Ok(subtract(&block, &cut)?)
}

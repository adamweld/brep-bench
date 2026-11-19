//! A ring sector, everything between 8 and 12 from the z axis with |z| ≤ 1 and polar angle from
//! 0° to 60°, cut by a bar leaning out 20° in the plane y = 0, its inner side facing the z axis
//! squarely, turned about the z axis from −140° to +140°. The bar is everything from the plane
//! through (10, 0, 0) square to (cos 20°, 0, −sin 20°) to 3 beyond it along that direction,
//! within 2 of the plane y = 0 and within 3 of the plane through (10, 0, 0) square to
//! (sin 20°, 0, cos 20°).
//!
//! solidcraft-kernel has no sweep of a solid, so the turn is built by elementary geometry. The
//! bar spans a few tens of degrees about the z axis, starts and ends clear of the block, and turns
//! far enough that every circle about the z axis in the block that meets it is passed over where
//! it crosses the block: what it takes from the block is its shadow, the circles it meets,
//! revolved. At height z (|z| ≤ 1.2) the bar is a rectangle: its inner side's line at
//! x = 10 + z tan 20°, |y| ≤ 2, its outer side's beyond x = 12.7, and its length's bounds farther
//! out still. Its nearest point to the axis is on the inner side at y = 0, so the shadow there is
//! everything from 10 + z tan 20° out past 12.5.
use super::drum_swung_partway_into_ring_sector::{ring_sector, turned};
use crate::solids::*;

pub fn build() -> R<Body> {
    let t = 20f64.to_radians().tan();
    // the shadow over |z| ≤ 1.2, past the block above and below and beyond its outer wall
    let shadow = polygon(&[(10.0 - 1.2 * t, -1.2), (12.5, -1.2), (12.5, 1.2), (10.0 + 1.2 * t, 1.2)]);
    let swept = turned(&revolve_z(shadow, 280.0)?, -140.0)?;
    cut(&ring_sector()?, &swept)
}

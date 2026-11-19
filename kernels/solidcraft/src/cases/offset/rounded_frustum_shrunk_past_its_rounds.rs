//! The cone frustum on the z axis from radius 6 at z = 0 to radius 3 at z = 6, both rims rounded
//! to radius 1.5, shrunk by 2: every point at least 2 from its outside, the kernel's offset inward
//! with the moved faces that cross trimmed and merged.
use super::rounded_puck_shrunk_by_its_rounds::round_circles;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_cone, Body};

/// The frustum with the rims `rims` (each `(radius, z)`: (6, 0) the bottom, (3, 6) the top)
/// rounded to 1.5.
pub fn frustum_rounded(rims: &[(f64, f64)]) -> Result<Body, Refusal> {
    round_circles(make_cone([0.0, 0.0, 0.0], 6.0, 3.0, 6.0)?, rims, 1.5)
}

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&frustum_rounded(&[(6.0, 0.0), (3.0, 6.0)])?, -2.0)
}

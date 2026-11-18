//! The cone on the z axis from its base of radius 6 at z = 0 towards its apex at z = 12, its tip
//! rounded by the ball of radius 1 tangent to its side, shrunk by 1.5: every point at least 1.5
//! from its outside, the kernel's offset inward with the moved faces that cross trimmed and
//! merged.
use crate::bench::{arc, line};
use crate::shared::geometry::{axis, turned};
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;
use std::f64::consts::FRAC_PI_2;

/// The ball-tipped cone, its half-section turned about the axis: the base, the side up to the
/// circle of radius 2/√5 at z = 12 − 4/√5 where the ball touches it, and the ball's arc about
/// (0, 12 − √5) on to the axis.
pub fn ball_tipped_cone() -> Result<Body, Refusal> {
    let s5 = 5f64.sqrt();
    let centre = 12.0 - s5;
    // angles in the xz plane from +x towards +z, about −y; the side touches the ball at atan2(1, 2)
    turned(vec![
        line([0.0, 0.0, 0.0], [6.0, 0.0, 0.0]),
        line([6.0, 0.0, 0.0], [2.0 / s5, 0.0, 12.0 - 4.0 / s5]),
        arc([0.0, 0.0, centre], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 1.0, 1f64.atan2(2.0), FRAC_PI_2),
        axis(centre + 1.0, 0.0),
    ])
}

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&ball_tipped_cone()?, -1.5)
}

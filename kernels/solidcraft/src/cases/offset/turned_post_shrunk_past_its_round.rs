//! A turned post on the z axis, the cylinder of radius 5 from z = 0 to z = 6 topped by the frustum
//! to radius 2 at z = 10, the edge between them rounded to radius 1, shrunk by 1.5: every point at
//! least 1.5 from its outside, the kernel's offset inward with the moved faces that cross trimmed
//! and merged.
use super::rounded_puck_shrunk_by_its_rounds::round_circles;
use crate::bench::line;
use crate::shared::geometry::{axis, turned};
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

/// The post, its half-section turned about the axis, then the circle of radius 5 at z = 6
/// rounded.
pub fn turned_post() -> Result<Body, Refusal> {
    let post = turned(vec![
        line([0.0, 0.0, 0.0], [5.0, 0.0, 0.0]),
        line([5.0, 0.0, 0.0], [5.0, 0.0, 6.0]),
        line([5.0, 0.0, 6.0], [2.0, 0.0, 10.0]),
        line([2.0, 0.0, 10.0], [0.0, 0.0, 10.0]),
        axis(10.0, 0.0),
    ])?;
    round_circles(post, &[(5.0, 6.0)], 1.0)
}

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&turned_post()?, -1.5)
}

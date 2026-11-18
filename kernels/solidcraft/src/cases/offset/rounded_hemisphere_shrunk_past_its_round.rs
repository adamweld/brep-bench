//! The half of the ball of radius 5 about the origin above z = 0, its rim rounded to radius 1,
//! shrunk by 3/2: every point at least 3/2 from its outside, the kernel's offset inward with the
//! moved faces that cross trimmed and merged.
use super::rounded_puck_shrunk_by_its_rounds::round_circles;
use crate::bench::cut;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_box, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    let below = make_box([-6.0, -6.0, -6.0], [12.0, 12.0, 6.0])?;
    let dome = cut(ball, below)?;
    offset_rounded(&round_circles(dome, &[(5.0, 0.0)], 1.0)?, -1.5)
}
